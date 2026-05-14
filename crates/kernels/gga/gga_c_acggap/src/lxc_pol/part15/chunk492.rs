//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 492/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk492<F: Float>(t34: F, t39: F, t100: F, t50: F, t277: F, t712: F, t43: F, t47: F, t52: F, t441: F, t939: F, t393: F, t157: F, t944: F, t2775: F, t2792: F, t2795: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2868 = t34 * t39;
    let t2876 = 1.0 / t100 / t50;
    let t2894 = t712 * t277;
    let t2896 = t43 * t43;
    let t2898 = 1.0 / t47 / t2896;
    let t2908 = t50 * t50;
    let t2910 = 1.0 / t52 / t2908;
    let t2929 = t939 * t441;
    let t2933 = t393 * t393;
    let t2934 = 1.0 / t2933;
    let t2937 = t944 * t157;
    let t2955 = t2792 * t2775 * t2795;
    (t2868, t2876, t2894, t2898, t2910, t2929, t2934, t2937, t2955)
}
