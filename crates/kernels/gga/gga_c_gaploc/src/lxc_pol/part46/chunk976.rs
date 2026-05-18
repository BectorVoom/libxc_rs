//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 976/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk976<F: Float>(t2615: F, t326: F, t43598: F, t41060: F, t41068: F, t41071: F, t41075: F, t41083: F, t41093: F, t2617: F, t3451: F, t7803: F) -> (F, F, F, F, F, F, F, F) {
    let t43601 = F::new(0.92023022289409799224e1) * t2615 * t326 * t43598;
    let t43602 = F::new(0.25561950635947166451e0) * t41060;
    let t43603 = F::new(0.11916829983950142223e0) * t41068;
    let t43604 = F::new(0.25561950635947166451e0) * t41071;
    let t43605 = F::new(0.59584149919750711116e-1) * t41075;
    let t43606 = F::new(0.17875244975925213335e0) * t41083;
    let t43607 = F::new(0.59584149919750711116e-1) * t41093;
    let t43609 = t7803 * t3451 * t2617;
    (t43601, t43602, t43603, t43604, t43605, t43606, t43607, t43609)
}
