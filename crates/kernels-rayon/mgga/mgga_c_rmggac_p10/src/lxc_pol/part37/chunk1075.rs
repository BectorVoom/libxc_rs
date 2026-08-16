//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1075/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1075(t1364: f64, t14980: f64, t1632: f64, t1635: f64, t3282: f64, t5898: f64, t69648: f64, t69663: f64, t69665: f64, t71419: f64, t77509: f64, t77510: f64, t77511: f64, t77512: f64, t77514: f64, t77515: f64, t77517: f64, t77519: f64, t77520: f64, t77521: f64, t884: f64, t903: f64) -> f64 {
    let t80242 = t77509 - t77510 - t77511 + t77512 - t77514 - t77515 - t77517 - t77519 - t77520 - t77521 + t71419 - 0.40878380883436523435e-5_f64 * t69648 + 0.17961362552795712846e0_f64 * t903 * t3282 * t1632 - 0.23948483403727617128e0_f64 * t1364 * t3282 * t1635 - 0.11974241701863808564e0_f64 * t884 * t14980 * t5898 - t69663 + t69665;
    t80242
}
