//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2005/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2005(t10836: f64, t25227: f64, t2661: f64, t2482: f64, t596: f64, t7036: f64, t2487: f64, t10832: f64, t25245: f64, t25266: f64, t2648: f64, t2681: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93031 = t2661 * t25227 * t10836;
    let t93034 = t2482 * t7036 * t596;
    let t93035 = t93034 * t2487;
    let t93043 = t25245 * t10832;
    let t93045 = t25266 * t2648;
    let t93048 = t820 * t7036 * t2681;
    (t93031, t93034, t93035, t93043, t93045, t93048)
}
