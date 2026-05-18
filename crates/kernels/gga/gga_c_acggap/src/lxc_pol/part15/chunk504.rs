//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 504/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk504<F: Float>(t277: F, t715: F, t43: F, t98: F, t34: F, t39: F, t100: F, t50: F, t712: F, t47: F, t52: F, t441: F, t939: F) -> (F, F, F, F, F, F, F, F) {
    let t2847 = t715 * t277;
    let t2861 = F::new(1.0) / t98 / t43;
    let t2868 = t34 * t39;
    let t2876 = F::new(1.0) / t100 / t50;
    let t2894 = t712 * t277;
    let t2896 = t43 * t43;
    let t2898 = F::new(1.0) / t47 / t2896;
    let t2908 = t50 * t50;
    let t2910 = F::new(1.0) / t52 / t2908;
    let t2929 = t939 * t441;
    (t2847, t2861, t2868, t2876, t2894, t2898, t2910, t2929)
}
