//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 900/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk900(t4028: f64, t8327: f64, t7458: f64, t1774: f64, t8326: f64, t652: f64, t1799: f64, t1998: f64, t59: f64, t6926: f64, t1825: f64, t6943: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32673 = t4028 * t8327;
    let t32674 = 2.0_f64 * t32673;
    let t32675 = t7458 * t8327;
    let t32676 = 2.0_f64 * t32675;
    let t32677 = t1774 * t8326;
    let t32678 = t652 * t32677;
    let t32679 = 2.0_f64 * t32678;
    let t32711 = t1998 * t59 * t1799;
    let t32712 = t6926 * t32711;
    let t32714 = t6943 * t1825;
    (t32674, t32676, t32677, t32679, t32711, t32712, t32714)
}
