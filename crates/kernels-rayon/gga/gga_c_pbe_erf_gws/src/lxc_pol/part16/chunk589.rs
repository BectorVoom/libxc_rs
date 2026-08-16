//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 589/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk589(t2641: f64, t1009: f64, t1651: f64, t587: f64, t247: f64, t2522: f64, t251: f64, t1061: f64, t719: f64, t256: f64, t19: f64, t991: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2642 = 8.0_f64 / 135.0_f64 * t2641;
    let t2643 = t1651 * t1009;
    let t2644 = t587 * t2643;
    let t2645 = 8.0_f64 / 135.0_f64 * t2644;
    let t2646 = t2522 * t247;
    let t2647 = t2646 * t251;
    let t2650 = t1061 * t719;
    let t2651 = t2650 * t256;
    let t2653 = t991 * t19;
    (t2642, t2643, t2645, t2646, t2647, t2650, t2651, t2653)
}
