//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 767/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk767<F: Float>(t11132: F, t240: F, t624: F, t281: F, t283: F, t3252: F, t276: F, t285: F, t273: F, t2922: F, t913: F, t275: F) -> (F, F, F, F, F, F, F, F) {
    let t11334 = F::new(0.93011851851851851854e0) * t11132;
    let t11335 = t624 * t240;
    let t11337 = t281 * t11335 * t283;
    let t11338 = F::new(0.36514074074074074075e0) * t11337;
    let t11341 = t240 * t3252;
    let t11354 = F::new(1.0) / t276 / t285 / F::new(4.0);
    let t11358 = F::new(1.0)/pow_3_2::<f64>(t273);
    let t11384 = F::new(1.0) / t2922 / t913;
    let t11385 = t275 * t11384;
    (t11334, t11335, t11337, t11338, t11341, t11354, t11358, t11385)
}
