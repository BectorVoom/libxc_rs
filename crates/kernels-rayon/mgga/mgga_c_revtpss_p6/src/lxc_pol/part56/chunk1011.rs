//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1011/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1011(t33462: f64, t34908: f64, t1769: f64, t8931: f64, t482: f64, t372: f64, t371: f64, t1243: f64, t1794: f64, t494: f64, t247: f64, t3719: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34909 = t33462 * t34908;
    let t34914 = t8931 * t1769;
    let t34915 = t33462 * t34914;
    let t34918 = t482 * t1769;
    let t34919 = t372 * t34918;
    let t34920 = t371 * t34919;
    let t34925 = t1243 * t1794;
    let t34929 = t494 * t1769;
    let t34931 = t247 * t3719 * t34929;
    (t34909, t34914, t34915, t34918, t34920, t34925, t34929, t34931)
}
