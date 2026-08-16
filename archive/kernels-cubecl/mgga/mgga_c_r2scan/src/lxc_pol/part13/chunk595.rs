//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 595/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk595<F: Float>(t2116: F, t3281: F, t502: F, t507: F, t512: F, t1050: F, t120: F, t1599: F, t526: F) -> (F, F, F, F) {
    let t3282 = t3281 * t2116;
    let t3283 = F::cast_from(0.27439371595564631661e-2_f64) * t3282;
    let t3285 = t512 * t502 * t507;
    let t3286 = F::cast_from(0.48787202696913915093e-2_f64) * t3285;
    let t3288 = t120 * t1599 * t1050;
    let t3289 = F::cast_from(0.64025200389650807209e-1_f64) * t3288;
    let t3290 = t120 * t526;
    (t3283, t3286, t3289, t3290)
}
