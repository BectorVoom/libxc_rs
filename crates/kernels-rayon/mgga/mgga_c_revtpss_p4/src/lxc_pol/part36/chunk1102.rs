//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1102/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1102(t24961: f64, t25014: f64, t1277: f64, t1211: f64, t24616: f64, t24633: f64, t1210: f64, t12628: f64, t1274: f64, t1770: f64, t1813: f64, t1829: f64, t20756: f64, t24892: f64, t24900: f64, t24906: f64, t3567: f64, t5220: f64, t5225: f64, t5251: f64, t5417: f64, t6564: f64, t6580: f64, t6588: f64, t6697: f64, t6703: f64, t6745: f64) -> (f64, f64, f64, f64, f64) {
    let t25015 = t24961 + t25014;
    let t25016 = t1277 * t25015;
    let t25019 = t1211 * t24616;
    let t25022 = t1211 * t24633;
    let t25025 = 0.19756347548806534796e1_f64 * t6564 * t1813 + 0.39512695097613069591e1_f64 * t5251 * t6580 + 0.39512695097613069591e1_f64 * t5225 * t6703 - 0.19756347548806534796e1_f64 * t5251 * t6588 + 0.19756347548806534796e1_f64 * t1770 * t6697 + 0.39512695097613069591e1_f64 * t3567 * t24892 + 0.39512695097613069591e1_f64 * t5417 * t6703 - 0.19756347548806534796e1_f64 * t5220 * t6588 + 0.19756347548806534796e1_f64 * t1210 * t24900 - 0.19756347548806534796e1_f64 * t5225 * t6745 - 0.39512695097613069591e1_f64 * t3567 * t24906 - 0.39512695097613069591e1_f64 * t20756 * t1829 - 0.65854491829355115987e0_f64 * t1274 * t25016 - 0.39512695097613069591e1_f64 * t12628 * t25019 - 0.65854491829355115987e0_f64 * t1210 * t25022;
    (t25015, t25016, t25019, t25022, t25025)
}
