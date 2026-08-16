//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 510/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk510(t2775: f64, t2792: f64, t2795: f64, t286: f64, t75: f64, t901: f64, t288: f64, t691: f64, t883: f64, t704: f64, t807: f64, t1: f64) -> (f64, f64, f64, f64, f64) {
    let t2955 = t2792 * t2775 * t2795;
    let t2956 = t286 * t2955;
    let t2957 = 0.10254018858216406658e4_f64 * t2956;
    let t2958 = t901 * t75;
    let t2959 = t2958 * t288;
    let t2961 = t883 * t691;
    let t2963 = t704 * t807;
    let t2965 = t901 * t1;
    (t2957, t2959, t2961, t2963, t2965)
}
