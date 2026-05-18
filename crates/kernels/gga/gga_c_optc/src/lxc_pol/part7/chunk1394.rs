//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1394/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1394<F: Float>(t3137: F, t3186: F, t3188: F, t9166: F, t27705: F, t9114: F, t3192: F, t3194: F, t1170: F, t1172: F, t3843: F, t3234: F, t9058: F, t9189: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t27778 = t3186 * t3137 * t3188;
    let t27780 = t9166 * sigma2;
    let t27781 = t27780 * t27705;
    let t27786 = t9114 * t27705;
    let t27792 = t3192 * t3137 * t3194;
    let t27795 = t1170 * t3843 * t1172;
    let t27798 = t3234 * t9189 * t9058;
    (t27778, t27781, t27786, t27792, t27795, t27798)
}
