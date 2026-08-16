//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1118/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1118(t20049: f64, t20053: f64, t20058: f64, t20063: f64, t20076: f64, t2359: f64, t2379: f64, t2388: f64, t2392: f64, t4405: f64, t4410: f64, t4459: f64, t6112: f64, t6135: f64, t6145: f64, t6789: f64, t6802: f64, t827: f64) -> f64 {
    let t20080 = -t6802 * t2379 / 24.0_f64 + t2388 * t6145 / 8.0_f64 - 35.0_f64 / 18.0_f64 * t20049 - t827 * t20053 / 4.0_f64 - t827 * t20058 / 12.0_f64 - t2359 * t20063 / 16.0_f64 - t6112 * t2379 / 24.0_f64 - t4405 * t4459 / 12.0_f64 - t4410 * t4459 / 12.0_f64 - t2392 * t6135 / 4.0_f64 - t2392 * t6789 / 8.0_f64 + 7.0_f64 / 24.0_f64 * t20076 + t2392 * t6145 / 8.0_f64;
    t20080
}
