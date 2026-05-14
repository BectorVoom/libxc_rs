//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 922/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk922<F: Float>(t24961: F, t25014: F, t1277: F, t1211: F, t24616: F, t24633: F, t1210: F, t12628: F, t1274: F, t1770: F, t1813: F, t1829: F, t20756: F, t24892: F, t24900: F, t24906: F, t3567: F, t5220: F, t5225: F, t5251: F, t5417: F, t6564: F, t6580: F, t6588: F, t6697: F, t6703: F, t6745: F) -> (F,) {
    let t25015 = t24961 + t25014;
    let t25016 = t1277 * t25015;
    let t25019 = t1211 * t24616;
    let t25022 = t1211 * t24633;
    let t25025 = 0.19756347548806534796e1 * t6564 * t1813 + 0.39512695097613069591e1 * t5251 * t6580 + 0.39512695097613069591e1 * t5225 * t6703 - 0.19756347548806534796e1 * t5251 * t6588 + 0.19756347548806534796e1 * t1770 * t6697 + 0.39512695097613069591e1 * t3567 * t24892 + 0.39512695097613069591e1 * t5417 * t6703 - 0.19756347548806534796e1 * t5220 * t6588 + 0.19756347548806534796e1 * t1210 * t24900 - 0.19756347548806534796e1 * t5225 * t6745 - 0.39512695097613069591e1 * t3567 * t24906 - 0.39512695097613069591e1 * t20756 * t1829 - 0.65854491829355115987e0 * t1274 * t25016 - 0.39512695097613069591e1 * t12628 * t25019 - 0.65854491829355115987e0 * t1210 * t25022;
    (t25025,)
}
