//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1037/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1037<F: Float>(t2912: F, t698: F, t11132: F, t240: F, t624: F, t281: F, t283: F, t2909: F, t3252: F, t276: F, t285: F, t273: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11326 = t698 * t2912;
    let t11334 = F::cast_from(0.93011851851851851854e0_f64) * t11132;
    let t11335 = t624 * t240;
    let t11337 = t281 * t11335 * t283;
    let t11338 = F::cast_from(0.36514074074074074075e0_f64) * t11337;
    let t11339 = t698 * t2909;
    let t11341 = t240 * t3252;
    let t11354 = F::cast_from(1.0_f64) / t276 / t285 / F::cast_from(4.0_f64);
    let t11358 = F::cast_from(1.0_f64)/pow_3_2::<F>(t273);
    (t11326, t11334, t11335, t11337, t11338, t11339, t11341, t11354, t11358)
}
