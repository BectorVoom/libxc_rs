//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1098/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1098(t13972: f64, t3993: f64, t13921: f64, t13925: f64, t13930: f64, t13939: f64, t13945: f64, t13948: f64, t13950: f64, t13955: f64, t13958: f64, t13962: f64, t13965: f64, t13966: f64, t13969: f64, t2384: f64, t2388: f64, t2392: f64, t4002: f64, t4385: f64, t6793: f64, t827: f64) -> (f64, f64) {
    let t13973 = t13972 * t3993;
    let t13974 = 7.0_f64 / 2304.0_f64 * t13973;
    let t13975 = -t13921 / 768.0_f64 + t4385 * t13925 / 96.0_f64 + t6793 * t13930 / 24.0_f64 - t2388 * t4002 / 96.0_f64 - t2392 * t4002 / 96.0_f64 - t827 * t13939 / 48.0_f64 + t13945 / 96.0_f64 - t13948 - t13950 / 24.0_f64 + t13955 - t13958 / 768.0_f64 - t2384 * t4002 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t13962 + t13965 + t13966 / 24.0_f64 - t13969 / 48.0_f64 + t13974;
    (t13973, t13975)
}
