//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2875/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2875(t15397: f64, t41583: f64, t2874: f64, t2918: f64, t4632: f64, t15534: f64, t3022: f64, t1100: f64, t3329: f64, t15537: f64, t3007: f64, t981: f64) -> (f64, f64, f64, f64, f64) {
    let t52182 = 0.1551780387578202009e4_f64 * t41583 * t15397;
    let t52185 = 6.0_f64 * t2874 * t4632 * t2918;
    let t52187 = 0.17544670867903938621e1_f64 * t3022 * t15534;
    let t52188 = t1100 * t3329;
    let t52194 = 0.35089341735807877242e1_f64 * t981 * t15537 * t3007;
    (t52182, t52185, t52187, t52188, t52194)
}
