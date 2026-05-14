//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 859/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk859<F: Float>(t17133: F, t17173: F, t17206: F, t17243: F, t106: F, t11130: F, t1411: F, t14472: F, t17079: F, t17092: F, t17096: F, t335: F, t3853: F, t3860: F, t4990: F, t5049: F, t7948: F, t908: F) -> (F, F) {
    let t17245 = t17133 + t17173 + t17206 + t17243;
    let t17249 = 0.27818116767324025134e1 * t106 * t17079 * t335 - 0.83454350301972075402e1 * t106 * t14472 * t1411 + 0.16690870060394415081e2 * t106 * t11130 * t4990 - 0.83454350301972075402e1 * t106 * t3853 * t5049 - 0.1669087006039441508e2 * t106 * t7948 * t17092 + 0.16690870060394415081e2 * t3860 * t17096 - 0.27818116767324025134e1 * t106 * t908 * t17245;
    (t17245, t17249)
}
