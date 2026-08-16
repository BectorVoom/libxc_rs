//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 532/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk532(t1815: f64, t2666: f64, t639: f64, t1675: f64, t256: f64, t2611: f64, t2614: f64, t2617: f64, t2619: f64, t2624: f64, t2629: f64, t2634: f64, t2639: f64, t2642: f64, t2645: f64, t2647: f64, t2651: f64, t2655: f64, t2657: f64, t2662: f64, t2664: f64, t2665: f64) -> (f64, f64, f64) {
    let t2667 = t1815 * t2666;
    let t2669 = 4.0_f64 / 45.0_f64 * t639 * t2667;
    let t2670 = -t2611 + t2614 + t2617 + t2619 + t2624 - t2629 + t2634 - t2639 + t2642 + t2645 + t2647 * t256 / 3.0_f64 + t2651 / 3.0_f64 + 0.60777777777777777777e-1_f64 * t2655 + 2.0_f64 / 9.0_f64 * t2657 + t2662 + t2664 - t1675 + t2665 - t2669;
    (t2667, t2669, t2670)
}
