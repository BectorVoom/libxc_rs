//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 917/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk917<F: Float>(t1035: F, t3016: F, t375: F, t3019: F, t388: F, t8561: F, t3053: F, t3058: F, t4219: F, t1102: F, t1084: F, t3057: F) -> (F, F, F, F, F, F, F, F) {
    let t8685 = F::new(1.0) / t3016 / t1035;
    let t8686 = t375 * t8685;
    let t8688 = F::new(1.0) / t3019 / t388;
    let t8689 = t8561 * t8688;
    let t8691 = F::new(0.51725014705706168417e3) * t8686 * t8689;
    let t8693 = t3058 * t3053 * t4219;
    let t8695 = F::new(0.51947267698127589897e2) * t1102 * t8693;
    let t8697 = F::new(1.0) / t3057 / t1084;
    (t8685, t8686, t8688, t8689, t8691, t8693, t8695, t8697)
}
