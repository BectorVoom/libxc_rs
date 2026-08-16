//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3030/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3030(t10760: f64, t40627: f64, t50613: f64, t14861: f64, t9794: f64, t10890: f64, t4458: f64, t10815: f64, t4426: f64, t40424: f64, t4430: f64, t14720: f64, t9775: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51089 = t10760 * t40627 * t50613;
    let t51092 = t10760 * t9794 * t14861;
    let t51095 = t10890 * t4458;
    let t51098 = t10815 * t4426;
    let t51100 = t40424 * t4430;
    let t51102 = t9775 * t14720;
    (t51089, t51092, t51095, t51098, t51100, t51102)
}
