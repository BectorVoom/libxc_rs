//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 525/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk525(t1621: f64, t2602: f64, t639: f64, t1615: f64, t1619: f64, t1669: f64, t2534: f64, t2535: f64, t2536: f64, t2551: f64, t2558: f64, t2564: f64, t2569: f64, t2574: f64, t2578: f64, t2583: f64, t2587: f64, t2590: f64, t2595: f64, t2600: f64, t267: f64) -> (f64, f64, f64) {
    let t2603 = t1621 * t2602;
    let t2605 = 4.0_f64 / 15.0_f64 * t639 * t2603;
    let t2606 = -2.0_f64 / 45.0_f64 * t1615 + t1619 + t2534 + t2535 - 2.0_f64 / 45.0_f64 * t2536 - t2551 * t267 / 15.0_f64 - t2558 + t2564 - t2569 + t2574 + t2578 + t2583 + t2587 + 2.0_f64 / 9.0_f64 * t1669 + t2590 - t2595 - t2600 + t2605;
    (t2603, t2605, t2606)
}
