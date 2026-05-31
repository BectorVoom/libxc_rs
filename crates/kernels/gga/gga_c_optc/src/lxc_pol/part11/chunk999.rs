//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 999/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk999<F: Float>(t1256: F, t1303: F, t443: F, t5239: F, t38: F, t22: F, t1784: F, t1793: F, t6433: F, t1757: F, t534: F, t36: F, t88: F) -> (F, F, F, F, F, F, F) {
    let t20063 = t1256 * t1303;
    let t20680 = t5239 * t443;
    let t20814 = t38 * t38;
    let t20816 = F::cast_from(1.0_f64) / t22 / t20814;
    let t21874 = F::cast_from(0.57894567559743977359e3_f64) * t6433 * t1793 * t1784;
    let t21875 = t1784 * t1784;
    let t21878 = F::cast_from(6.0_f64) * t1757 * t21875 * t534;
    let t21881 = F::cast_from(1.0_f64) / t20814;
    let t21884 = F::cast_from(840.0_f64) * t36 * t21881 * t88;
    (t20063, t20680, t20816, t21874, t21875, t21878, t21884)
}
