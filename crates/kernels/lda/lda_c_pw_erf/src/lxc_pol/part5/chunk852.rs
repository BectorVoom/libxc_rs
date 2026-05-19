//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 852/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk852<F: Float>(t756: F, t770: F, t2765: F, t2642: F, t454: F, t142: F, t1809: F, t2610: F, t5504: F, t5519: F, t767: F, t1820: F, t1826: F, t2329: F, t2337: F, t3234: F, t3243: F, t406: F, t408: F, t7354: F, t7360: F, t7365: F, t7370: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7880 = t770 * t756;
    let t7881 = t2765 * t7880;
    let t7886 = t454 * t2642;
    let t7887 = t7886 * t142;
    let t7889 = t1809 * t2610;
    let t7893 = F::cast_from(1.9486833333333333_f64) * t5504;
    let t7896 = F::cast_from(0.9743416666666667_f64) * t5519;
    let t7897 = t767 * t2610;
    let t7913 = F::new(4.0) / F::new(27.0) * t3234 * t7354 - t1820 * t2329 / F::new(3.0) + t406 * t7360 / F::new(3.0) + F::new(4.0) / F::new(27.0) * t3243 * t7365 - t1826 * t2337 / F::new(3.0) + t408 * t7370 / F::new(3.0);
    (t7880, t7881, t7886, t7887, t7889, t7893, t7896, t7897, t7913)
}
