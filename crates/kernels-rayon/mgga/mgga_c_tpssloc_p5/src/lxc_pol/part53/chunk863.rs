//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 863/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk863(t31056: f64, t1266: f64, t8326: f64, t652: f64, t1307: f64, t3701: f64, t6920: f64, t8462: f64, t1998: f64, t59: f64, t6926: f64, t6600: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31057 = 2.0_f64 * t31056;
    let t31058 = t1266 * t8326;
    let t31059 = t652 * t31058;
    let t31060 = 2.0_f64 * t31059;
    let t31085 = t3701 * t1307;
    let t31153 = t6920 * t8462;
    let t31156 = t1998 * t59 * t1307;
    let t31157 = t6926 * t31156;
    let t31159 = t6600 * t8462;
    (t31057, t31058, t31060, t31085, t31153, t31156, t31157, t31159)
}
