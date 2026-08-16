//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 633/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk633(t3379: f64, t41: f64, t2641: f64, t2644: f64, t1044: f64, t1792: f64, t186: f64, t211: f64, t1675: f64, t1988: f64, t2002: f64, t2006: f64, t2009: f64, t2960: f64, t2965: f64, t2971: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3380 = t41 * t3379;
    let t3388 = 16.0_f64 / 135.0_f64 * t2641;
    let t3389 = 16.0_f64 / 135.0_f64 * t2644;
    let t3390 = t1044 * t1044;
    let t3391 = t1792 * t3390;
    let t3392 = t186 * t3391;
    let t3394 = 4.0_f64 / 15.0_f64 * t211 * t3392;
    let t3396 = t1988 + 8.0_f64 / 3.0_f64 * t2960 + 8.0_f64 / 3.0_f64 * t2965 + t2002 + t2006 + t2009 + t3388 + t3389 + t3394 + 0.21642082724729686754e0_f64 * t2971 - t1675;
    (t3380, t3388, t3389, t3390, t3391, t3392, t3394, t3396)
}
