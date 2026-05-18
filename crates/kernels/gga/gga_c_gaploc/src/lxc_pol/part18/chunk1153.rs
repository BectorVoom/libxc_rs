//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1153/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1153<F: Float>(t31158: F, t9561: F, t1397: F, t6851: F, t9562: F, t21005: F, t544: F, t6466: F, t900: F, t549: F, t6520: F, t7025: F) -> (F, F, F, F, F, F) {
    let t31160 = F::new(0.3575048995185042667e0) * t9561 * t31158;
    let t31163 = F::new(0.17875244975925213335e0) * t1397 * t6851 * t9562;
    let t31166 = F::new(0.17875244975925213335e0) * t544 * t21005 * t9562;
    let t31167 = t900 * t6466;
    let t31169 = F::new(0.89376224879626066674e-1) * t9561 * t31167;
    let t31172 = F::new(0.11916829983950142223e0) * t7025 * t549 * t6520;
    (t31160, t31163, t31166, t31167, t31169, t31172)
}
