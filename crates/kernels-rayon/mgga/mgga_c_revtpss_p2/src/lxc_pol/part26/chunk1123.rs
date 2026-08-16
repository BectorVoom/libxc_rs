//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1123/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1123(t25266: f64, t2756: f64, t10836: f64, t25227: f64, t2661: f64, t2482: f64, t596: f64, t7036: f64, t2487: f64, t10820: f64, t7045: f64, t10863: f64, t25262: f64) -> (f64, f64, f64, f64, f64) {
    let t93028 = t25266 * t2756;
    let t93031 = t2661 * t25227 * t10836;
    let t93034 = t2482 * t7036 * t596;
    let t93035 = t93034 * t2487;
    let t93037 = t7045 * t10820;
    let t93039 = t25262 * t10863;
    (t93028, t93031, t93035, t93037, t93039)
}
