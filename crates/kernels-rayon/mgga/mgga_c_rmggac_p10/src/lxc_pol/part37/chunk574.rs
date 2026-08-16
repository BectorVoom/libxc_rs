//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 574/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk574(t14980: f64, t352: f64, t1356: f64, t14168: f64, t14217: f64, t14220: f64, t3292: f64, t504: f64, t14234: f64, t14241: f64, t14246: f64, t14256: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14981 = t14980 * t352;
    let t14982 = t1356 * t14981;
    let t14983 = 0.39914139006212695214e-1_f64 * t14982;
    let t14984 = 0.58171619854173713844e-5_f64 * t14168;
    let t14987 = 0.32526727992809621482e-5_f64 * t14217;
    let t14988 = 0.32526727992809621482e-5_f64 * t14220;
    let t14989 = t504 * t3292;
    let t14990 = 0.19957069503106347607e-1_f64 * t14989;
    let t14991 = 0.72714524817717142305e-5_f64 * t14234;
    let t14993 = 0.58171619854173713844e-5_f64 * t14241;
    let t14994 = 0.17451485956252114153e-4_f64 * t14246;
    let t14995 = 0.58171619854173713844e-5_f64 * t14256;
    (t14981, t14983, t14984, t14987, t14988, t14990, t14991, t14993, t14994, t14995)
}
