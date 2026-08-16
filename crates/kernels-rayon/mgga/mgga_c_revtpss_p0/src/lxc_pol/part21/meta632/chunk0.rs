//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2400/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2400(t40861: f64, t802: f64, t10899: f64, t794: f64, t10902: f64, t159: f64, t216: f64, t2475: f64, t2645: f64, t860: f64, t231: f64, t2782: f64, t2783: f64, t39714: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40862 = t40861 * t802;
    let t40864 = t794 * t10899;
    let t40865 = t40864 * t10902;
    let t40868 = t216 * t159 * t2475;
    let t40888 = t860 * t2645;
    let t40894 = t2782 * t2783 * t39714 * t231;
    (t40862, t40864, t40865, t40868, t40888, t40894)
}
