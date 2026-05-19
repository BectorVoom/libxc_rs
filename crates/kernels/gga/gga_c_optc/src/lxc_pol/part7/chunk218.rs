//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 218/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk218<F: Float>(t43: F, t50: F, t601: F, t603: F, t103: F, t172: F, t47: F, t549: F, t52: F, t553: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t605 = F::cast_from(0.58482233974552040708e0_f64) * t601 * t603;
    let t606 = t103 * t172;
    let t607 = F::new(1.0) / t47;
    let t610 = piecewise3::<F>(t44, F::new(0.0), F::new(2.0) / F::new(3.0) * t607 * t549);
    let t611 = F::new(1.0) / t52;
    let t614 = piecewise3::<F>(t51, F::new(0.0), F::new(2.0) / F::new(3.0) * t611 * t553);
    let t616 = t610 / F::new(2.0) + t614 / F::new(2.0);
    (t605, t606, t607, t611, t616)
}
