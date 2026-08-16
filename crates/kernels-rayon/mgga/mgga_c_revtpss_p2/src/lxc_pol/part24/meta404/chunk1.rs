//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1342/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1342(t225: f64, t40603: f64, t785: f64, t2737: f64, t853: f64, t9794: f64, t10292: f64, t66: f64, t240: f64, t10688: f64, t243: f64, t268: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40609 = t40603 * t785 * t225;
    let t40611 = 0.63807336860547134325e-3_f64 * t40609 * t2737;
    let t40627 = t9794 * t853;
    let t40633 = 1.0_f64 / t66 / t10292;
    let t40634 = t40633 * t240;
    let t40638 = 0.53552153920316253184e-5_f64 * t10688 * t40634 * t243 * t268;
    (t40609, t40611, t40627, t40633, t40634, t40638)
}
