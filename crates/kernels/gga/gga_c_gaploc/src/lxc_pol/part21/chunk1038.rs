//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1038/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1038<F: Float>(t21005: F, t544: F, t9562: F, t6466: F, t900: F, t9561: F, t549: F, t6520: F, t7025: F, t18736: F, t20540: F, t2365: F, t20692: F, t4130: F, t874: F, t6907: F, t9272: F) -> (F, F, F, F, F, F, F) {
    let t31166 = 0.17875244975925213335e0 * t544 * t21005 * t9562;
    let t31167 = t900 * t6466;
    let t31169 = 0.89376224879626066674e-1 * t9561 * t31167;
    let t31172 = 0.11916829983950142223e0 * t7025 * t549 * t6520;
    let t31175 = 0.59584149919750711116e-1 * t18736 * t2365 * t20540;
    let t31178 = 0.59584149919750711116e-1 * t7025 * t2365 * t20692;
    let t31187 = t4130 * t874;
    let t31190 = 0.10352590007558602413e2 * t9272 * t31187 * t6907;
    (t31166, t31167, t31169, t31172, t31175, t31178, t31190)
}
