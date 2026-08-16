//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1882/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1882(t14701: f64, t92955: f64, t241: f64, t820: f64, t93060: f64, t4447: f64, t92951: f64, t14727: f64, t25227: f64, t2661: f64, t4430: f64, t93034: f64) -> (f64, f64, f64, f64, f64) {
    let t98983 = t92955 * t14701;
    let t98988 = t820 * t93060 * t241;
    let t98991 = t92951 * t4447;
    let t99000 = t2661 * t25227 * t14727;
    let t99002 = t93034 * t4430;
    (t98983, t98988, t98991, t99000, t99002)
}
