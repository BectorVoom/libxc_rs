//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1239/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1239(t2482: f64, t596: f64, t7036: f64, t2487: f64, t10820: f64, t7045: f64, t10863: f64, t25262: f64, t10828: f64, t7038: f64, t10832: f64, t25245: f64) -> (f64, f64, f64, f64, f64) {
    let t93034 = t2482 * t7036 * t596;
    let t93035 = t93034 * t2487;
    let t93037 = t7045 * t10820;
    let t93039 = t25262 * t10863;
    let t93041 = t7038 * t10828;
    let t93043 = t25245 * t10832;
    (t93035, t93037, t93039, t93041, t93043)
}
