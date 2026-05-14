//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 844/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk844<F: Float>(t1250: F, t8232: F, t1091: F, t2894: F, t835: F, t2749: F, t4167: F, t840: F, t4129: F, t875: F, t871: F, t1212: F, t2801: F, t1882: F, t4164: F, t4169: F) -> (F, F, F, F, F, F, F) {
    let t15147 = t8232 * t1250;
    let t15150 = t835 * t2894 * t1091;
    let t15154 = t840 * t2749 * t4167;
    let t15157 = t4129 * t875;
    let t15159 = t840 * t871 * t15157;
    let t15162 = t1212 * t2801;
    let t15164 = t840 * t871 * t15162;
    let t15168 = 4.0 / 9.0 * t1882 * t4164;
    let t15170 = 2.0 / 9.0 * t1882 * t4169;
    (t15147, t15150, t15154, t15159, t15164, t15168, t15170)
}
