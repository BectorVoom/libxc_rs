//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1097/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1097(t22909: f64, t9820: f64, t2624: f64, t7419: f64, t9800: f64, t1967: f64, t22044: f64, t23104: f64, t883: f64, t2673: f64, t7503: f64, t23469: f64, t9787: f64) -> (f64, f64, f64, f64, f64) {
    let t28443 = 0.29792074959875355558e-1_f64 * t9820 * t22909;
    let t28449 = t9800 * t2624 * t7419;
    let t28450 = 0.72851559312449424384e1_f64 * t28449;
    let t28453 = t23104 * t1967 * t883 * t22044;
    let t28454 = 0.76685851907841499352e0_f64 * t28453;
    let t28529 = 0.17875244975925213335e0_f64 * t2673 * t7503;
    let t28563 = t23469 * t9787;
    (t28443, t28450, t28454, t28529, t28563)
}
