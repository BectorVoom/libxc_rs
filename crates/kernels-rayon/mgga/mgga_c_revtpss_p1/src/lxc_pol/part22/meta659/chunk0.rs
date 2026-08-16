//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2614/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2614(t20921: f64, t4181: f64, t12787: f64, t12916: f64, t6689: f64, t3718: f64, t17661: f64, t5401: f64, t1214: f64, t1715: f64, t1250: f64, t17353: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20922 = t20921 * t4181;
    let t20923 = t12787 * t20922;
    let t20926 = t12916 * t6689;
    let t20927 = t3718 * t20926;
    let t20929 = t17661 * t5401;
    let t20932 = t1715 * t1214;
    let t20933 = t1250 * t20932;
    let t20934 = t17353 * t20933;
    (t20922, t20923, t20926, t20927, t20929, t20932, t20933, t20934)
}
