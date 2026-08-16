//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 835/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk835(t2036: f64, t785: f64, t133: f64, t5913: f64, t793: f64, t2009: f64, t306: f64, t5931: f64, t287: f64, t6012: f64, t6010: f64, t2128: f64, t2131: f64, t2135: f64, t2140: f64, t290: f64, t2981: f64, t5989: f64, t6009: f64, t6014: f64, t6017: f64, t6021: f64, t6023: f64, t6026: f64, t6028: f64, t6031: f64, t791: f64, t794: f64) -> (f64, f64, f64) {
    let t6036 = t2036 * t785;
    let t6039 = t5913 * t133;
    let t6040 = t6039 * t793;
    let t6043 = t306 * t2009;
    let t6047 = t5931 * t306;
    let t6048 = t6012 * t287;
    let t6049 = t6010 * t6048;
    let t6054 = 0.39512695097613069591e1_f64 * t6009 * t6014 + 0.39512695097613069591e1_f64 * t6017 * t2128 + 0.39512695097613069591e1_f64 * t6021 * t6023 - 0.39512695097613069591e1_f64 * t6026 * t6028 + 0.19756347548806534796e1_f64 * t6031 * t794 + 0.19756347548806534796e1_f64 * t2131 * t2135 - 0.19756347548806534796e1_f64 * t6036 * t2140 + 0.65854491829355115987e0_f64 * t791 * t6040 - 0.19756347548806534796e1_f64 * t2036 * t6043 * t2981 + 0.65854491829355115987e0_f64 * t6047 * t6049 + 0.65854491829355115987e0_f64 * t290 * t5989;
    (t6040, t6049, t6054)
}
