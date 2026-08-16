//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1072/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1072(t124604: f64, t7642: f64, t127: f64, t33509: f64, t33510: f64, t371: f64, t33412: f64, t8938: f64, t97346: f64, t124610: f64, t3781: f64, t482: f64, t494: f64) -> (f64, f64, f64, f64, f64) {
    let t124626 = t7642 * t124604;
    let t124632 = t33509 * t371 * t127 * t33510;
    let t124635 = t8938 * t97346 * t33412;
    let t124644 = t7642 * t3781 * t124610;
    let t124645 = t482 * t494;
    (t124626, t124632, t124635, t124644, t124645)
}
