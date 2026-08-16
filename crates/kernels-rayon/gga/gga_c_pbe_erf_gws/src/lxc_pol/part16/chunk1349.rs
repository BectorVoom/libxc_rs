//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1349/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1349(t14888: f64, t15036: f64, t19704: f64, t20113: f64, t29845: f64, t52188: f64, t52529: f64, t53945: f64, t53950: f64, t53963: f64, t53966: f64, t53968: f64, t55717: f64, t55722: f64, t55726: f64, t55729: f64, t55734: f64, t6793: f64, t8629: f64, t8793: f64) -> f64 {
    let t55738 = t8629 * t52188 / 48.0_f64 + t8793 * t52529 / 48.0_f64 + t53945 / 128.0_f64 + t19704 * t15036 / 48.0_f64 + t19704 * t14888 / 48.0_f64 + t20113 * t15036 / 48.0_f64 + t6793 * t55717 / 24.0_f64 + t6793 * t55722 / 24.0_f64 + t53950 / 12.0_f64 + t55726 + 5.0_f64 / 192.0_f64 * t53963 - t53966 / 24.0_f64 - t29845 * t55729 / 32.0_f64 - t6793 * t55734 / 12.0_f64 + t53968 / 12.0_f64;
    t55738
}
