//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1374/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1374(t853: f64, t9794: f64, t775: f64, t837: f64, t10760: f64, t10292: f64, t66: f64, t240: f64, t10688: f64, t243: f64, t268: f64, t2694: f64, t9784: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40627 = t9794 * t853;
    let t40628 = t837 * t775;
    let t40630 = t10760 * t40627 * t40628;
    let t40633 = 1.0_f64 / t66 / t10292;
    let t40634 = t40633 * t240;
    let t40638 = 0.53552153920316253184e-5_f64 * t10688 * t40634 * t243 * t268;
    let t40639 = t9784 * t2694;
    (t40628, t40630, t40633, t40634, t40638, t40639)
}
