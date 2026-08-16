//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1091/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1091(t43: f64, t2092: f64, t4347: f64, t311: f64, t19: f64, t2331: f64, t301: f64, t305: f64, t799: f64, t19059: f64, t19062: f64, t19064: f64, t19066: f64, t19068: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t19528 = t2092 * t4347;
    let t19529 = 0.73024584604562962965e1_f64 * t19528;
    let t19530 = t311 * t311;
    let t19537 = 0.34072858057724757727e0_f64 * t305 / t19530 * t2331 * t301 * t19 * t799;
    let t19544 = piecewise3(t44, 0.0_f64, -56.0_f64 / 81.0_f64 * t19059 + 16.0_f64 / 9.0_f64 * t19062 - 2.0_f64 / 3.0_f64 * t19064 - 8.0_f64 / 9.0_f64 * t19066 + 2.0_f64 / 3.0_f64 * t19068);
    (t19529, t19537, t19544)
}
