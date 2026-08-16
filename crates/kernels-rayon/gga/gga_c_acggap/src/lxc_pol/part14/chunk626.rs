//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 626/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk626(t2606: f64, t2610: f64, t2613: f64, t2616: f64, t2622: f64, t2625: f64, t2629: f64, t2634: f64, t2642: f64, t2644: f64, t2826: f64, t5443: f64, t5444: f64, t5476: f64, t5477: f64, t5478: f64) -> f64 {
    let t6004 = -t2606 + t2610 + t5443 + t2613 + t2616 + t5444 - t2622 - t2625 + t5476 + t2629 - t2634 + t5477 + t5478 - t2642 + t2644 + t2826;
    t6004
}
