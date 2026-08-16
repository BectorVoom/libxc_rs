//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 989/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk989(t2558: f64, t3049: f64, t943: f64, t2936: f64, t7671: f64, t1897: f64, t8942: f64, t954: f64, t3440: f64, t7129: f64, t8637: f64, t948: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10749 = t3049 * t2558;
    let t10750 = t943 * t10749;
    let t10751 = 0.32043859292259267849e-3_f64 * t10750;
    let t10752 = t2936 * t7671;
    let t10754 = 0.23071578690426672851e-1_f64 * t1897 * t10752;
    let t10755 = t954 * t8942;
    let t10757 = 0.76905262301422242837e-2_f64 * t1897 * t10755;
    let t10759 = 0.23071578690426672851e-1_f64 * t7129 * t3440;
    let t10760 = t8637 * t948;
    (t10749, t10751, t10752, t10754, t10755, t10757, t10759, t10760)
}
