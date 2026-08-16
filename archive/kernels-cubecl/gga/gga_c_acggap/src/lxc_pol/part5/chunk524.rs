//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 524/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk524<F: Float>(t100: F, t50: F, t277: F, t712: F, t43: F, t47: F, t52: F, t1210: F, t394: F, t441: F, t939: F, t393: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2876 = F::cast_from(1.0_f64) / t100 / t50;
    let t2894 = t712 * t277;
    let t2896 = t43 * t43;
    let t2898 = F::cast_from(1.0_f64) / t47 / t2896;
    let t2908 = t50 * t50;
    let t2910 = F::cast_from(1.0_f64) / t52 / t2908;
    let t2925 = t394 * t1210;
    let t2929 = t939 * t441;
    let t2933 = t393 * t393;
    (t2876, t2894, t2896, t2898, t2908, t2910, t2925, t2929, t2933)
}
