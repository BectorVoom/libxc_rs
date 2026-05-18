//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 835/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk835<F: Float>(t2036: F, t785: F, t133: F, t5913: F, t793: F, t2009: F, t306: F, t5931: F, t287: F, t6012: F, t6010: F, t2128: F, t2131: F, t2135: F, t2140: F, t290: F, t2981: F, t5989: F, t6009: F, t6014: F, t6017: F, t6021: F, t6023: F, t6026: F, t6028: F, t6031: F, t791: F, t794: F) -> (F, F, F) {
    let t6036 = t2036 * t785;
    let t6039 = t5913 * t133;
    let t6040 = t6039 * t793;
    let t6043 = t306 * t2009;
    let t6047 = t5931 * t306;
    let t6048 = t6012 * t287;
    let t6049 = t6010 * t6048;
    let t6054 = F::new(0.39512695097613069591e1) * t6009 * t6014 + F::new(0.39512695097613069591e1) * t6017 * t2128 + F::new(0.39512695097613069591e1) * t6021 * t6023 - F::new(0.39512695097613069591e1) * t6026 * t6028 + F::new(0.19756347548806534796e1) * t6031 * t794 + F::new(0.19756347548806534796e1) * t2131 * t2135 - F::new(0.19756347548806534796e1) * t6036 * t2140 + F::new(0.65854491829355115987e0) * t791 * t6040 - F::new(0.19756347548806534796e1) * t2036 * t6043 * t2981 + F::new(0.65854491829355115987e0) * t6047 * t6049 + F::new(0.65854491829355115987e0) * t290 * t5989;
    (t6040, t6049, t6054)
}
