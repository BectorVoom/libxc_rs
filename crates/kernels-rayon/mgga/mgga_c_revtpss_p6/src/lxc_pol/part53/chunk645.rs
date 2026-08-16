//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 645/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk645(t1444: f64, t1903: f64, t4076: f64, t1882: f64, t555: f64, t4086: f64, t543: f64, t2782: f64, t1883: f64, t72: f64, t686: f64, t4101: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5727 = t1903 * t1444;
    let t5728 = t4076 * t5727;
    let t5735 = t555 * t1882;
    let t5737 = t4086 * t5735 * t543;
    let t5738 = t2782 * t5737;
    let t5740 = t1883 * t72;
    let t5741 = t5740 * t686;
    let t5742 = t4101 * t5741;
    (t5727, t5728, t5735, t5738, t5741, t5742)
}
