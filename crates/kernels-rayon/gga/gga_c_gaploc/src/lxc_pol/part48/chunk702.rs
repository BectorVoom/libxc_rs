//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 702/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk702(t13296: f64, t600: f64, t568: f64, t13322: f64, t531: f64, t13423: f64, t13424: f64, t13425: f64, t13428: f64, t13430: f64, t13436: f64, t13440: f64, t13442: f64, t13444: f64, t13446: f64, t1562: f64, t193: f64, t557: f64, t597: f64) -> (f64, f64, f64, f64) {
    let t13449 = t600 * t13296;
    let t13450 = t568 * t13449;
    let t13453 = t531 * t13322;
    let t13456 = -t13423 - t13424 - t13425 - t13428 - 0.13803453343411469884e2_f64 * t1562 * t13430 + t13436 - t13440 - t13442 - t13444 + 0.35750489951850426669e0_f64 * t13446 * t193 + 0.23005755572352449806e1_f64 * t597 * t13450 - 0.35750489951850426669e0_f64 * t557 * t13453;
    (t13449, t13450, t13453, t13456)
}
