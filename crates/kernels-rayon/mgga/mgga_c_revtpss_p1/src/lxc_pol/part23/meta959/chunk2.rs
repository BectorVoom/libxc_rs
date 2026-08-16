//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3223/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3223(t1234: f64, t1248: f64, t12709: f64, t12723: f64, t12756: f64, t1280: f64, t1285: f64, t1287: f64, t1794: f64, t1818: f64, t20800: f64, t21342: f64, t21557: f64, t24986: f64, t24989: f64, t3670: f64, t3783: f64, t5412: f64, t5478: f64, t5494: f64, t59241: f64, t59864: f64, t59865: f64, t6564: f64, t6622: f64, t6714: f64, t70209: f64, t82207: f64, t82321: f64, t82886: f64, t84175: f64) -> f64 {
    let t84778 = 0.19756347548806534796e1_f64 * t12756 * t82321 * t3783 - 0.65854491829355115987e0_f64 * t1234 * t1280 * t84175 - 0.19756347548806534796e1_f64 * t12709 * t24986 - 0.19756347548806534796e1_f64 * t12723 * t24986 + 0.39512695097613069591e1_f64 * t3670 * t1280 * t82207 + 0.19756347548806534796e1_f64 * t1285 * t5412 * t6622 * t1287 + 0.39512695097613069591e1_f64 * t59241 * t6714 - 0.19756347548806534796e1_f64 * t12709 * t24989 + 0.19756347548806534796e1_f64 * t6564 * t5494 - 0.19756347548806534796e1_f64 * t5478 * t20800 * t21557 - 0.19756347548806534796e1_f64 * t70209 * t1818 + 0.19756347548806534796e1_f64 * t1285 * t21342 * t1794 * t1287 + 0.15805078039045227836e2_f64 * t59864 * t82886 * t59865 * t1248;
    t84778
}
