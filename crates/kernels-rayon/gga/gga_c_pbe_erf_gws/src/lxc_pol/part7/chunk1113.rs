//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1113/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1113(t2246: f64, t4438: f64, t2417: f64, t8734: f64, t829: f64, t830: f64, t19608: f64, t19659: f64, t19744: f64, t19756: f64, t19907: f64, t19914: f64, t19923: f64, t2306: f64, t2384: f64, t2391: f64, t2392: f64, t2408: f64, t2409: f64, t3074: f64, t4390: f64, t4419: f64, t6106: f64, t6135: f64, t6755: f64, t6781: f64, t6784: f64, t6789: f64, t6822: f64, t827: f64, t833: f64, t8606: f64, t9283: f64) -> f64 {
    let t19925 = t2246 * t4438;
    let t19937 = t8734 * t2417;
    let t19939 = t829 * t830 * t19937;
    let t19950 = -7.0_f64 / 6.0_f64 * t19907 + t19659 * t4390 / 6.0_f64 - t19608 * t19914 / 6.0_f64 + t2392 * t4419 / 16.0_f64 + t6106 * t2391 * t833 / 32.0_f64 - 7.0_f64 / 72.0_f64 * t19923 + 7.0_f64 / 36.0_f64 * t19925 - t2384 * t6135 / 4.0_f64 - t2384 * t6789 / 8.0_f64 - t2384 * t6784 / 8.0_f64 + t3074 * t2306 * t19744 * t8606 / 8.0_f64 + t827 * t19939 / 4.0_f64 + t2408 * t2409 * t6781 * t6755 / 4.0_f64 - t2408 * t9283 * t19756 * t6822 / 2.0_f64;
    t19950
}
