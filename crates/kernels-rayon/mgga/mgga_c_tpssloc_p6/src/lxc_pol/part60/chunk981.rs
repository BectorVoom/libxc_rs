//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 981/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk981(t1998: f64, t59: f64, t6347: f64, t6926: f64, t22845: f64, t6330: f64, t22827: f64, t28100: f64, t6943: f64, t6415: f64, t6936: f64, t6378: f64, t8465: f64, t8467: f64) -> (f64, f64, f64, f64, f64) {
    let t127263 = t6926 * t1998 * t59 * t6347;
    let t127267 = t22845 * t1998 * t59 * t6330;
    let t127270 = t22827 * t6943 * t28100;
    let t127273 = t6936 * t6943 * t6415;
    let t127278 = t6378 * t8465 * t8467;
    (t127263, t127267, t127270, t127273, t127278)
}
