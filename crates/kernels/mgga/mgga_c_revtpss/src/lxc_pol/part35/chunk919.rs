//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 919/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk919<F: Float>(t13149: F, t24911: F, t5486: F, t6587: F, t1280: F, t24713: F, t13129: F, t1774: F, t21541: F, t24616: F, t1234: F, t1285: F, t12987: F, t13127: F, t13142: F, t13148: F, t17934: F, t1818: F, t1822: F, t1825: F, t20850: F, t21439: F, t24912: F, t24915: F, t24919: F, t24922: F, t24928: F, t3670: F, t460: F, t5326: F, t5436: F, t6564: F, t6720: F, t6727: F, t6731: F, t6735: F) -> (F,) {
    let t24931 = t24911 * t13149;
    let t24934 = t5486 * t6587;
    let t24941 = t1280 * t24713;
    let t24948 = t24911 * t13129;
    let t24951 = t21541 * t1774;
    let t24956 = t1280 * t24616;
    let t24961 = -0.39512695097613069591e1 * t13142 * t24912 + 0.65854491829355115987e0 * t460 * t24915 + 0.19756347548806534796e1 * t1285 * t24919 + 0.39512695097613069591e1 * t3670 * t24922 + 0.39512695097613069591e1 * t5436 * t6731 + 0.19756347548806534796e1 * t1285 * t24928 + 0.39512695097613069591e1 * t13148 * t24931 - 0.19756347548806534796e1 * t1234 * t24934 + 0.19756347548806534796e1 * t5436 * t6735 + 0.19756347548806534796e1 * t21439 * t1822 + 0.39512695097613069591e1 * t3670 * t24941 - 0.19756347548806534796e1 * t20850 * t1818 - 0.39512695097613069591e1 * t5326 * t6720 + 0.65854491829355115987e0 * t13127 * t24948 - 0.19756347548806534796e1 * t1234 * t24951 + 0.39512695097613069591e1 * t17934 * t6727 - 0.39512695097613069591e1 * t12987 * t24956 + 0.19756347548806534796e1 * t6564 * t1825;
    (t24961,)
}
