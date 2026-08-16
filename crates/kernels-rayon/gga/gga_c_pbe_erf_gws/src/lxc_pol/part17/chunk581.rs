//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 581/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk581(t2554: f64, t418: f64, t1821: f64, t587: f64, t1661: f64, t197: f64, t1663: f64, t950: f64, t1017: f64, t572: f64, t1827: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2555 = t2554 * t418;
    let t2556 = t1821 * t2555;
    let t2558 = 8.0_f64 / 45.0_f64 * t587 * t2556;
    let t2559 = t1661 * t197;
    let t2560 = t1663 * t950;
    let t2561 = t2560 * t418;
    let t2562 = t2559 * t2561;
    let t2564 = 4.0_f64 / 27.0_f64 * t587 * t2562;
    let t2565 = t1017 * t572;
    let t2566 = t2565 * t418;
    let t2567 = t1827 * t2566;
    let t2569 = 4.0_f64 / 45.0_f64 * t587 * t2567;
    (t2555, t2556, t2558, t2559, t2560, t2561, t2562, t2564, t2565, t2566, t2567, t2569)
}
