//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1077/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1077<F: Float>(t1280: F, t24616: F, t1234: F, t1285: F, t12987: F, t13127: F, t13142: F, t13148: F, t17934: F, t1818: F, t1822: F, t1825: F, t20850: F, t21439: F, t24912: F, t24915: F, t24919: F, t24922: F, t24928: F, t24931: F, t24934: F, t24941: F, t24948: F, t24951: F, t3670: F, t460: F, t5326: F, t5436: F, t6564: F, t6720: F, t6727: F, t6731: F, t6735: F) -> F {
    let t24956 = t1280 * t24616;
    let t24961 = -F::cast_from(0.39512695097613069591e1_f64) * t13142 * t24912 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t24915 + F::cast_from(0.19756347548806534796e1_f64) * t1285 * t24919 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t24922 + F::cast_from(0.39512695097613069591e1_f64) * t5436 * t6731 + F::cast_from(0.19756347548806534796e1_f64) * t1285 * t24928 + F::cast_from(0.39512695097613069591e1_f64) * t13148 * t24931 - F::cast_from(0.19756347548806534796e1_f64) * t1234 * t24934 + F::cast_from(0.19756347548806534796e1_f64) * t5436 * t6735 + F::cast_from(0.19756347548806534796e1_f64) * t21439 * t1822 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t24941 - F::cast_from(0.19756347548806534796e1_f64) * t20850 * t1818 - F::cast_from(0.39512695097613069591e1_f64) * t5326 * t6720 + F::cast_from(0.65854491829355115987e0_f64) * t13127 * t24948 - F::cast_from(0.19756347548806534796e1_f64) * t1234 * t24951 + F::cast_from(0.39512695097613069591e1_f64) * t17934 * t6727 - F::cast_from(0.39512695097613069591e1_f64) * t12987 * t24956 + F::cast_from(0.19756347548806534796e1_f64) * t6564 * t1825;
    t24961
}
