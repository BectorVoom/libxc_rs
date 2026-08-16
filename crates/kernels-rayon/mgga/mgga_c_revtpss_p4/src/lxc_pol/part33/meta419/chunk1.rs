//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1493/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1493(t2723: f64, t4423: f64, t4364: f64, t4365: f64, t231: f64, t4343: f64, t2747: f64, t10779: f64, t14671: f64, t6035: f64, t10777: f64, t14676: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18632 = t2723 * t4423;
    let t18634 = t4364 * t4365 * t18632;
    let t18637 = t231 * t4343;
    let t18639 = t2747 * t4365 * t18637;
    let t18643 = t10779 * t14671 * t6035;
    let t18644 = t10777 * t18643;
    let t18647 = t2747 * t14676 * t6035;
    (t18632, t18634, t18639, t18643, t18644, t18647)
}
