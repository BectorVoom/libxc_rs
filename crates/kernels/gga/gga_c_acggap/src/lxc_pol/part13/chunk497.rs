//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 497/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk497<F: Float>(t2933: F, t157: F, t944: F, t1258: F, t377: F, t2775: F, t2792: F, t2795: F, t286: F, t75: F, t901: F, t288: F, t691: F, t883: F, t704: F, t807: F) -> (F, F, F, F, F, F, F) {
    let t2934 = 1.0 / t2933;
    let t2937 = t944 * t157;
    let t2946 = t377 * t1258;
    let t2955 = t2792 * t2775 * t2795;
    let t2956 = t286 * t2955;
    let t2957 = 0.10254018858216406658e4 * t2956;
    let t2958 = t901 * t75;
    let t2959 = t2958 * t288;
    let t2961 = t883 * t691;
    let t2963 = t704 * t807;
    (t2934, t2937, t2946, t2957, t2959, t2961, t2963)
}
