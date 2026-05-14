//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 791/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk791<F: Float>(t2615: F, t326: F, t43598: F, t41060: F, t41068: F, t41071: F, t41075: F, t41083: F, t41093: F, t13154: F, t24799: F, t24661: F, t13153: F, t3251: F, t4752: F, t13023: F, t2103: F, t4673: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t43601 = 0.92023022289409799224e1 * t2615 * t326 * t43598;
    let t43602 = 0.25561950635947166451e0 * t41060;
    let t43603 = 0.11916829983950142223e0 * t41068;
    let t43604 = 0.25561950635947166451e0 * t41071;
    let t43605 = 0.59584149919750711116e-1 * t41075;
    let t43606 = 0.17875244975925213335e0 * t41083;
    let t43607 = 0.59584149919750711116e-1 * t41093;
    let t43617 = 0.42900587942220512003e1 * t24799 * t13154;
    let t43619 = 0.42900587942220512003e1 * t24661 * t13154;
    let t43627 = 0.28600391961480341335e1 * t13153 * t4752 * t3251;
    let t43630 = 0.47667319935800568892e0 * t2103 * t4673 * t13023;
    (t43601, t43602, t43603, t43604, t43605, t43606, t43607, t43617, t43619, t43627, t43630)
}
