//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 644/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk644(t1721: f64, t401: f64, t1715: f64, t25: f64, t5022: f64, t5025: f64, t5030: f64, t5034: f64, t5039: f64, t5042: f64, t5047: f64, t5049: f64, t5052: f64) -> f64 {
    let t5054 = t401 * t1721;
    let t5056 = t401 * t1715;
    let t5058 = -0.26666666666666666667e-1_f64 * t5022 + 0.13333333333333333333e-1_f64 * t25 * t5025 - 0.66666666666666666666e-2_f64 * t25 * t5030 - 0.39999999999999999999e-1_f64 * t25 * t5034 + 0.39999999999999999999e-1_f64 * t25 * t5039 - 0.71983333333333333333e-1_f64 * t5042 - t5047 - 0.66666666666666666667e-2_f64 * t25 * t5049 - 0.22222222222222222222e-1_f64 * t5052 + 0.13333333333333333334e-1_f64 * t5054 + 0.44444444444444444445e-2_f64 * t5056;
    t5058
}
