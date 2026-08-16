//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3071/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3071(t15068: f64, t51120: f64, t11185: f64, t18677: f64, t1098: f64, t18245: f64, t1119: f64, t18686: f64, t3308: f64, t3312: f64, t5983: f64, t3316: f64) -> (f64, f64, f64, f64, f64) {
    let t63745 = 0.1034520258385468006e4_f64 * t51120 * t15068;
    let t63747 = 12.0_f64 * t11185 * t18677;
    let t63750 = t18245 * t1098;
    let t63752 = 2.0_f64 * t63750 * t1119;
    let t63754 = 1.0_f64 * t18686 * t3308;
    let t63755 = t5983 * t3312;
    let t63757 = 0.16081979498692535067e2_f64 * t63755 * t3316;
    (t63745, t63747, t63752, t63754, t63757)
}
