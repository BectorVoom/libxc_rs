//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 910/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk910(t10167: f64, t10176: f64, t10132: f64, t10134: f64, t10144: f64, t10147: f64, t10151: f64, t138: f64, t1572: f64, t1577: f64, t2902: f64, t2905: f64, t2919: f64, t3675: f64, t3683: f64, t514: f64, t520: f64, t5847: f64, t5854: f64, t8206: f64, t8209: f64, t985: f64) -> f64 {
    let t10177 = t10167 + t10176;
    let t10179 = t10132 * t138 - t10134 * t520 - 6.0_f64 * t10144 * t5854 + 4.0_f64 * t10147 * t1577 + 2.0_f64 * t10151 * t1577 - t10177 * t514 - t1572 * t3683 - 2.0_f64 * t2902 * t2919 + 4.0_f64 * t2905 * t8209 + 2.0_f64 * t3675 * t5847 - 2.0_f64 * t8206 * t985;
    t10179
}
