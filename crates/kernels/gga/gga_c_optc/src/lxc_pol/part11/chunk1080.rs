//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1080/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1080<F: Float>(t1179: F, t18102: F, t2586: F, t12635: F, t15873: F, t17855: F, t438: F, t17921: F, t4457: F, t45343: F, t3103: F, t44014: F, t5324: F, t18030: F, t3244: F, t9142: F) -> (F, F, F, F, F, F) {
    let t54797 = t1179 * t2586 * t18102;
    let t54799 = t12635 * t15873;
    let t54837 = t17855 * t438;
    let t54843 = t4457 * t45343 * t17921;
    let t54846 = t3103 * t44014 * t5324;
    let t54850 = t3244 * t9142 * t18030;
    (t54797, t54799, t54837, t54843, t54846, t54850)
}
