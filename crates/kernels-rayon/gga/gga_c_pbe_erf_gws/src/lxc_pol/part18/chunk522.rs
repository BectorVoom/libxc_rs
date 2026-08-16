//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 522/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk522(t2559: f64, t2561: f64, t587: f64, t1017: f64, t572: f64, t418: f64, t1827: f64, t1022: f64, t626: f64, t422: f64, t1809: f64, t1620: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2562 = t2559 * t2561;
    let t2564 = 4.0_f64 / 27.0_f64 * t587 * t2562;
    let t2565 = t1017 * t572;
    let t2566 = t2565 * t418;
    let t2567 = t1827 * t2566;
    let t2569 = 4.0_f64 / 45.0_f64 * t587 * t2567;
    let t2570 = t1022 * t626;
    let t2571 = t2570 * t422;
    let t2572 = t1809 * t2571;
    let t2574 = 8.0_f64 / 45.0_f64 * t1620 * t2572;
    (t2562, t2564, t2566, t2567, t2569, t2570, t2571, t2572, t2574)
}
