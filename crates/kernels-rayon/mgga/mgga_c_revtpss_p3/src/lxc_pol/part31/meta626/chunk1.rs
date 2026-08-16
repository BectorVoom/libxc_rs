//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2079/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2079(t1032: f64, t4930: f64, t994: f64, t15669: f64, t1976: f64, t1035: f64, t1983: f64, t99682: f64, t25698: f64, t93920: f64, t1647: f64, t7135: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99708 = t4930 * t1032;
    let t99709 = t994 * t99708;
    let t99721 = t15669 * t1976;
    let t99743 = t1983 * t99682 * t1035;
    let t99824 = t25698 * t93920;
    let t99881 = t1647 * t7135;
    (t99708, t99709, t99721, t99743, t99824, t99881)
}
