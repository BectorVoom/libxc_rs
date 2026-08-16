//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1558/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1558(t243: f64, t816: f64, t9707: f64, t813: f64, t2394: f64, t2476: f64, t236: f64, t807: f64, t2689: f64, t2694: f64, t2430: f64, t854: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10671 = t9707 * t243 * t816;
    let t10673 = 0.12846167376791569079e-2_f64 * t813 * t10671;
    let t10674 = t2476 * t2394;
    let t10675 = t236 * t10674;
    let t10676 = t807 * t10675;
    let t10678 = t2689 * t2694;
    let t10680 = t854 * t2430;
    (t10673, t10674, t10675, t10676, t10678, t10680)
}
