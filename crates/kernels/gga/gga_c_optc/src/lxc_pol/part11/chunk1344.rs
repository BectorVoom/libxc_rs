//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1344/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1344<F: Float>(t43: F, t55912: F, t1492: F, t53885: F, t1567: F, t870: F, t15008: F, t15012: F, t15016: F, t1579: F, t17443: F, t17465: F, t17534: F, t17543: F, t17635: F, t4230: F, t4536: F, t5098: F, t5103: F, t53918: F, t8287: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t58252 = piecewise3::<F>(t44, F::new(0.0), t55912);
    let t58265 = F::cast_from(0.23392893589820816284e1_f64) * t53885 * t1492;
    let t58284 = t1567 * t870;
    let t58293 = F::new(2.0) / F::new(3.0) * t53918 * t1579 + t15012 * t5098 + F::new(56.0) / F::new(27.0) * t4536 * t17443 - F::new(16.0) / F::new(3.0) * t15008 * t5098 - F::new(16.0) / F::new(9.0) * t4230 * t17635 - F::new(448.0) / F::new(81.0) * t4230 * t17443 + F::new(352.0) / F::new(27.0) * t15016 * t5103 + F::new(128.0) / F::new(9.0) * t4230 * t17465 + F::cast_from(16000000.0_f64) / F::new(243.0) * t8287 * t58284 * t17534 * t17543 - F::new(16.0) / F::new(3.0) * t4536 * t17465 + F::new(88.0) / F::new(9.0) * t15016 * t5098;
    (t58252, t58265, t58293)
}
