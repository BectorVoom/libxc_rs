//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 778/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk778(t3040: f64, t360: f64, t1021: f64, t248: f64, t1030: f64, t372: f64, t364: f64, t354: f64) -> (f64, f64, f64, f64) {
    let t3041 = t3040 * t360;
    let t3043 = t248 * t1021 * t3041;
    let t3046 = t1030 * t372;
    let t3047 = t364 * t3046;
    let t3048 = t354 * t3047;
    (t3041, t3043, t3047, t3048)
}
