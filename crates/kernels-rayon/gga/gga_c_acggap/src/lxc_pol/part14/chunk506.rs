//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 506/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk506(t393: f64, t157: f64, t944: f64, t2775: f64, t2792: f64, t2795: f64, t286: f64, t691: f64, t883: f64, t704: f64, t807: f64, t2868: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2933 = t393 * t393;
    let t2934 = 1.0_f64 / t2933;
    let t2937 = t944 * t157;
    let t2955 = t2792 * t2775 * t2795;
    let t2956 = t286 * t2955;
    let t2957 = 0.10254018858216406658e4_f64 * t2956;
    let t2961 = t883 * t691;
    let t2963 = t704 * t807;
    let t2968 = t2868 * t88;
    (t2934, t2937, t2957, t2961, t2963, t2968)
}
