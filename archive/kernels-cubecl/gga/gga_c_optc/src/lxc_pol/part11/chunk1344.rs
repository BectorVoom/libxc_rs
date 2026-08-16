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
    let t58252 = piecewise3::<F>(t44, F::cast_from(0.0_f64), t55912);
    let t58265 = F::cast_from(0.23392893589820816284e1_f64) * t53885 * t1492;
    let t58284 = t1567 * t870;
    let t58293 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t53918 * t1579 + t15012 * t5098 + F::cast_from(56.0_f64) / F::cast_from(27.0_f64) * t4536 * t17443 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t15008 * t5098 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t4230 * t17635 - F::cast_from(448.0_f64) / F::cast_from(81.0_f64) * t4230 * t17443 + F::cast_from(352.0_f64) / F::cast_from(27.0_f64) * t15016 * t5103 + F::cast_from(128.0_f64) / F::cast_from(9.0_f64) * t4230 * t17465 + F::cast_from(16000000.0_f64) / F::cast_from(243.0_f64) * t8287 * t58284 * t17534 * t17543 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t4536 * t17465 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t15016 * t5098;
    (t58252, t58265, t58293)
}
