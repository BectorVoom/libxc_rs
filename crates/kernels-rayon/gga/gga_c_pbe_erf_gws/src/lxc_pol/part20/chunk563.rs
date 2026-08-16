//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 563/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk563(t1: f64, t3: f64, t991: f64, t672: f64, t2009: f64, t2590: f64, t2595: f64, t2600: f64, t2605: f64, t2611: f64, t2614: f64, t2617: f64, t2619: f64, t2624: f64, t2629: f64, t2634: f64, t2639: f64, t2642: f64, t2645: f64, t2662: f64, t2664: f64) -> (f64, f64, f64) {
    let t2970 = t991 * t1 * t3;
    let t2971 = t2970 * t672;
    let t2973 = t2009 + t2590 - t2595 - t2600 + t2605 - t2611 + t2614 + t2617 + t2619 + t2624 - t2629 + t2634 - t2639 + t2642 + t2645 + 0.10821041362364843377e0_f64 * t2971 + t2662 + t2664;
    (t2970, t2971, t2973)
}
