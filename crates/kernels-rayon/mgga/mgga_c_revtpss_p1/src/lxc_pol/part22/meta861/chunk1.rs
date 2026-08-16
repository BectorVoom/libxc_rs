//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3012/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3012(t14769: f64, t2652: f64, t10716: f64, t14757: f64, t14772: f64, t221: f64, t2674: f64, t40683: f64, t1558: f64, t2645: f64, t10868: f64, t2482: f64, t814: f64) -> (f64, f64, f64, f64, f64) {
    let t50529 = t2652 * t14769;
    let t50531 = t10716 * t14757;
    let t50538 = t221 * t14772;
    let t50540 = t2674 * t40683 * t50538;
    let t50560 = t1558 * t2645;
    let t50570 = t2482 * t10868 * t814;
    (t50529, t50531, t50540, t50560, t50570)
}
