//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 950/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk950(t20: f64, t2653: f64, t2004: f64, t5919: f64, t5922: f64, t7179: f64, t7180: f64, t7184: f64, t7185: f64, t7187: f64, t7190: f64, t7193: f64, t7198: f64, t7203: f64, t7208: f64, t7215: f64, t7221: f64, t7223: f64) -> f64 {
    let t8424 = t2653 * t20;
    let t8425 = t8424 * t2004;
    let t8427 = -t7179 - t7180 + t7184 + t7185 + t7187 + 0.11181742741110338156e-1_f64 * t8425 - t5919 + t5922 - t7190 + t7193 - t7198 + t7203 + t7208 - t7215 + t7221 + t7223;
    t8427
}
