//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1391/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1391<F: Float>(t34188: F, t34191: F, t34176: F, t34178: F, t34193: F, t34200: F, t36849: F, t36850: F, t36851: F, t36854: F, t36855: F, t34205: F) -> (F, F) {
    let t36856 = F::cast_from(0.3437982149563945044e-8_f64) * t34188;
    let t36857 = F::cast_from(0.2845640240200497334e-7_f64) * t34191;
    let t36860 = t36849 + t36850 + t36851 + F::cast_from(0.3623181683912940217e-6_f64) * t34176 + F::cast_from(0.7246363367825880434e-6_f64) * t34178 - t36854 + t36855 + t36856 - t36857 + F::cast_from(0.73794894748263888896e-6_f64) * t34193 - F::cast_from(0.38527756621470067412e-7_f64) * t34200;
    let t36862 = F::cast_from(0.40481770833333333336e-4_f64) * t34205;
    (t36860, t36862)
}
