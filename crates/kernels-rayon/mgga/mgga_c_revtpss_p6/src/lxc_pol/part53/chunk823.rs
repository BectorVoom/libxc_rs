//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 823/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk823(t784: f64, t1425: f64, t240: f64, t2712: f64, t136: f64, t1412: f64, t220: f64, t4010: f64, t72: f64, t245: f64, t1384: f64, t138: f64, t2438: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9644 = t784 * t784;
    let t9645 = 1.0_f64 / t9644;
    let t9655 = t1425 * t1425;
    let t9656 = 1.0_f64 / t9655;
    let t9794 = t2712 * t240;
    let t9817 = t1412 * t136;
    let t9818 = t9817 * t220;
    let t9954 = t4010 * t72;
    let t9955 = t9954 * t245;
    let t9989 = t1384 * t1384;
    let t9990 = 1.0_f64 / t9989;
    let t10073 = t138 * t2438 * t785;
    (t9644, t9645, t9655, t9656, t9794, t9818, t9955, t9990, t10073)
}
