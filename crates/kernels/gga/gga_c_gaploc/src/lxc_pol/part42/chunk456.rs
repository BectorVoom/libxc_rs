//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 456/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk456<F: Float>(t129: F, t2276: F, t1242: F, t1232: F, t130: F, t1234: F, t136: F, t137: F, t4074: F, t4077: F, t4082: F, t4085: F, t2282: F, t3101: F, t3106: F, t467: F) -> (F, F, F, F, F, F, F, F) {
    let t9099 = t129 * t2276;
    let t9100 = t9099 * t1242;
    let t9102 = t130 * t1232;
    let t9103 = t1234 * t136;
    let t9105 = 1.0 / t137 / t9103;
    let t9106 = t9102 * t9105;
    let t9108 = t9106 * t4074 * t4077;
    let t9111 = t4082 * t9106 * t4085;
    let t9113 = t3101 * t2282;
    let t9115 = t3106 * t467;
    (t9099, t9100, t9105, t9106, t9108, t9111, t9113, t9115)
}
