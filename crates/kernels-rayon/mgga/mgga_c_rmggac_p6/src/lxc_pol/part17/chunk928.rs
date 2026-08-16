//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 928/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk928(t34828: f64, t9864: f64, t511: f64, t6477: f64, t34884: f64, t9845: f64, t1965: f64, t9824: f64, t1969: f64, t1973: f64, t1756: f64, t1971: f64, t495: f64, t515: f64, t7230: f64) -> (f64, f64, f64, f64, f64) {
    let t45466 = t34828 * t9864;
    let t45468 = t6477 * t511;
    let t45469 = t45468 * t9864;
    let t45473 = t34884 * t9845;
    let t45475 = t9824 * t1965;
    let t45476 = t45475 * t1969;
    let t45477 = t45476 * t1973;
    let t45482 = t7230 * t1971 * t515 * t1756 * t495;
    (t45466, t45469, t45473, t45477, t45482)
}
