//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2008/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2008(t240: f64, t25260: f64, t10728: f64, t2661: f64, t2479: f64, t25222: f64, t25228: f64, t9775: f64, t10732: f64, t25227: f64, t10705: f64, t25234: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93082 = t25260 * t240;
    let t93084 = t2661 * t93082 * t10728;
    let t93086 = t25222 * t2479;
    let t93088 = t9775 * t25228;
    let t93091 = t2661 * t25227 * t10732;
    let t93095 = t25234 * t10705;
    (t93082, t93084, t93086, t93088, t93091, t93095)
}
