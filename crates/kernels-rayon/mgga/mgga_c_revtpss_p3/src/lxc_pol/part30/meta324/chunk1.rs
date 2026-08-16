//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1325/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1325(t10689: f64, t268: f64, t10688: f64, t2479: f64, t2652: f64, t207: f64, t242: f64, t240: f64, t72: f64, t136: f64, t2476: f64, t221: f64, t2394: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10690 = t10689 * t268;
    let t10692 = 0.20082057720118594944e-6_f64 * t10688 * t10690;
    let t10693 = t2652 * t2479;
    let t10696 = 1.0_f64 / t242 / t207;
    let t10697 = t240 * t10696;
    let t10698 = t10697 * t72;
    let t10703 = t2476 * t136;
    let t10705 = t10703 * t221 * t2394;
    (t10690, t10692, t10693, t10698, t10703, t10705)
}
