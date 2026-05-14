//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 857/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk857<F: Float>(t23598: F, t996: F, t1695: F, t3269: F, t6392: F, t1651: F, t6350: F, t1079: F, t6258: F, t1076: F, t1647: F, t1652: F, t16600: F, t1696: F, t19351: F, t20178: F, t20204: F, t20211: F, t23583: F, t3058: F, t4778: F, t4935: F, t6245: F, t6251: F, t6259: F, t6345: F, t6351: F, t995: F) -> (F,) {
    let t23599 = t996 * t23598;
    let t23603 = t3269 * t1695 * t6392;
    let t23607 = t3269 * t1651 * t6350;
    let t23616 = t1651 * t6392;
    let t23617 = t1079 * t23616;
    let t23620 = t6258 * t1695;
    let t23621 = t1079 * t23620;
    let t23628 = -0.19756347548806534796e1 * t20204 * t1652 + 0.39512695097613069591e1 * t16600 * t6245 - 0.19756347548806534796e1 * t4778 * t6259 - 0.39512695097613069591e1 * t3058 * t23583 - 0.19756347548806534796e1 * t20211 * t1652 - 0.65854491829355115987e0 * t995 * t23599 + 0.39512695097613069591e1 * t1076 * t23603 - 0.39512695097613069591e1 * t995 * t23607 - 0.19756347548806534796e1 * t19351 * t1696 - 0.19756347548806534796e1 * t20178 * t1696 + 0.39512695097613069591e1 * t4935 * t6351 + 0.19756347548806534796e1 * t995 * t23617 + 0.19756347548806534796e1 * t995 * t23621 + 0.39512695097613069591e1 * t4778 * t6251 + 0.19756347548806534796e1 * t1647 * t6345;
    (t23628,)
}
