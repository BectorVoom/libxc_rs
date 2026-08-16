//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3214/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3214(t5091: f64, t11947: f64, t6270: f64, t193: f64, t336: f64, t3637: f64, t3640: f64, t4700: f64, t64436: f64, t64441: f64, t65301: f64, t65305: f64, t65307: f64, t65309: f64, t65312: f64, t65314: f64, t65319: f64, t65321: f64, t65324: f64, t65326: f64) -> f64 {
    let t66892 = t5091 * t5091;
    let t66897 = t6270 * t11947;
    let t66901 = -2.0_f64 * t193 * t336 * t3640 * t66892 + 2.0_f64 * t3637 * t4700 * t66897 + t64436 - t64441 - t65301 + t65305 + t65307 - t65309 + t65312 - t65314 - t65319 - t65321 - t65324 - t65326;
    t66901
}
