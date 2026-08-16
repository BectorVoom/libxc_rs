//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 559/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk559(t10743: f64, t943: f64, t2549: f64, t3437: f64, t2558: f64, t3049: f64, t2936: f64, t7671: f64, t1897: f64, t8942: f64, t954: f64, t3440: f64, t7129: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10744 = t943 * t10743;
    let t10745 = 0.32043859292259267849e-3_f64 * t10744;
    let t10746 = t2549 * t3437;
    let t10747 = 0.32043859292259267849e-3_f64 * t10746;
    let t10749 = t3049 * t2558;
    let t10750 = t943 * t10749;
    let t10751 = 0.32043859292259267849e-3_f64 * t10750;
    let t10752 = t2936 * t7671;
    let t10754 = 0.23071578690426672851e-1_f64 * t1897 * t10752;
    let t10755 = t954 * t8942;
    let t10757 = 0.76905262301422242837e-2_f64 * t1897 * t10755;
    let t10759 = 0.23071578690426672851e-1_f64 * t7129 * t3440;
    (t10744, t10745, t10746, t10747, t10750, t10751, t10754, t10757, t10759)
}
