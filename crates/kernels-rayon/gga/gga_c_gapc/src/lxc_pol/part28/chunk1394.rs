//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1394/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1394(t34188: f64, t34191: f64, t34176: f64, t34178: f64, t34193: f64, t34200: f64, t36849: f64, t36850: f64, t36851: f64, t36854: f64, t36855: f64, t34205: f64) -> (f64, f64) {
    let t36856 = 0.3437982149563945044e-8_f64 * t34188;
    let t36857 = 0.2845640240200497334e-7_f64 * t34191;
    let t36860 = t36849 + t36850 + t36851 + 0.3623181683912940217e-6_f64 * t34176 + 0.7246363367825880434e-6_f64 * t34178 - t36854 + t36855 + t36856 - t36857 + 0.73794894748263888896e-6_f64 * t34193 - 0.38527756621470067412e-7_f64 * t34200;
    let t36862 = 0.40481770833333333336e-4_f64 * t34205;
    (t36860, t36862)
}
