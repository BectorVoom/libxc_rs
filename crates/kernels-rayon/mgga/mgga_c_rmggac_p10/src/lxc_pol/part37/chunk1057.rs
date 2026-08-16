//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1057/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1057(t74523: f64, t14980: f64, t1550: f64, t1624: f64, t1627: f64, t3282: f64, t739: f64, t74508: f64, t74511: f64, t74514: f64, t74517: f64, t74520: f64, t77069: f64, t77070: f64, t77075: f64, t77077: f64, t77081: f64, t77082: f64, t77083: f64, t77084: f64, t8377: f64, t903: f64) -> f64 {
    let t80118 = 0.82834157616596963771e-1_f64 * t74523;
    let t80128 = -0.32526727992809621482e-5_f64 * t74508 - 0.32526727992809621482e-5_f64 * t74511 - 0.32526727992809621482e-5_f64 * t74514 - 0.32526727992809621482e-5_f64 * t74517 + t74520 + t80118 - t77069 + t77070 - t77075 - t77077 - t77081 + t77082 - t77083 - 0.11974241701863808564e0_f64 * t1550 * t3282 * t1624 + 0.17961362552795712846e0_f64 * t903 * t3282 * t1627 + 0.11974241701863808564e0_f64 * t739 * t14980 * t8377 + t77084;
    t80128
}
