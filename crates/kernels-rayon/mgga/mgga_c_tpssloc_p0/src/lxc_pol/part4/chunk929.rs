//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 929/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk929(t13012: f64, t4130: f64, t2563: f64, t4138: f64, t4134: f64, t9546: f64, t118: f64, t4119: f64, t794: f64, t2576: f64, t225: f64, t4266: f64) -> (f64, f64, f64, f64, f64) {
    let t13014 = 0.23333333333333333332e-1_f64 * t13012 * t4130;
    let t13020 = t2563 * t4138;
    let t13022 = t9546 * t4134;
    let t13025 = t118 * t794 * t4119;
    let t13027 = 0.16666666666666666666e-2_f64 * t2576 * t13025;
    let t13042 = t4266 * t225;
    (t13014, t13020, t13022, t13027, t13042)
}
