//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 506/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk506<F: Float>(t393: F, t157: F, t944: F, t2775: F, t2792: F, t2795: F, t286: F, t691: F, t883: F, t704: F, t807: F, t2868: F, t88: F) -> (F, F, F, F, F, F) {
    let t2933 = t393 * t393;
    let t2934 = F::new(1.0) / t2933;
    let t2937 = t944 * t157;
    let t2955 = t2792 * t2775 * t2795;
    let t2956 = t286 * t2955;
    let t2957 = F::new(0.10254018858216406658e4) * t2956;
    let t2961 = t883 * t691;
    let t2963 = t704 * t807;
    let t2968 = t2868 * t88;
    (t2934, t2937, t2957, t2961, t2963, t2968)
}
