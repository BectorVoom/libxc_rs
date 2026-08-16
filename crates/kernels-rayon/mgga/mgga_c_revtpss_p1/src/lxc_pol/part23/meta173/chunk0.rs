//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1038/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1038(t4893: f64, t4900: f64, t3117: f64, t1043: f64, t1651: f64, t1045: f64, t357: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4901 = t4893 * t4900;
    let t4902 = t3117 * t4901;
    let t4905 = t1651 * t1043;
    let t4906 = t4905 * t1045;
    let t4907 = t3117 * t4906;
    let t4910 = t357 * t999;
    (t4901, t4902, t4905, t4906, t4907, t4910)
}
