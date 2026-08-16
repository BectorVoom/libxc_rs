//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2017/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2017(t2022: f64, t22: f64, t25937: f64, t94696: f64, t7282: f64, t93139: f64, t1955: f64, t25920: f64, t4075: f64, t2435: f64, t26061: f64, t1385: f64, t7274: f64) -> (f64, f64, f64, f64, f64) {
    let t94698 = t25937 * t2022 * t22;
    let t94700 = 0.43639970290213137151e-3_f64 * t94696 * t94698;
    let t94701 = t93139 * t7282;
    let t94703 = 0.51727911450665971904e-3_f64 * t94701 * t94698;
    let t94705 = t1955 * t25920 * t4075;
    let t94714 = t2435 * t26061;
    let t94716 = t1385 * t7274;
    (t94700, t94703, t94705, t94714, t94716)
}
