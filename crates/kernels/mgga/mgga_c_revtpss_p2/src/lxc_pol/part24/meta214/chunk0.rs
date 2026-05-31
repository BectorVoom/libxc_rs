//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 958/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk958<F: Float>(t287: F, t2922: F, t275: F, t11132: F, t240: F, t624: F, t281: F, t283: F, t3252: F, t276: F, t285: F, t273: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11298 = F::cast_from(1.0_f64) / t2922 / t287;
    let t11299 = t275 * t11298;
    let t11304 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t11132;
    let t11334 = F::cast_from(0.93011851851851851854e0_f64) * t11132;
    let t11335 = t624 * t240;
    let t11337 = t281 * t11335 * t283;
    let t11338 = F::cast_from(0.36514074074074074075e0_f64) * t11337;
    let t11341 = t240 * t3252;
    let t11354 = F::cast_from(1.0_f64) / t276 / t285 / F::cast_from(4.0_f64);
    let t11358 = F::cast_from(1.0_f64)/pow_3_2::<F>(t273);
    (t11298, t11299, t11304, t11334, t11335, t11337, t11338, t11341, t11354, t11358)
}
