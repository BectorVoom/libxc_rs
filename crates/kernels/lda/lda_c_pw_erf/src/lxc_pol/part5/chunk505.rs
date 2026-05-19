//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 505/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk505<F: Float>(t2480: F, t548: F, t1268: F, t2429: F, t2433: F, t538: F, t2437: F, t1240: F, t1263: F, t1964: F, t2087: F, t2431: F, t2435: F, t2439: F, t25: F) -> (F, F, F, F, F) {
    let t2482 = F::new(4.0) / F::new(15.0) * t548 * t2480;
    let t2488 = t1268 * t2429;
    let t2491 = t538 * t2433;
    let t2494 = t538 * t2437;
    let t2497 = t1240 + F::cast_from(0.023994444444444443_f64) * t1964 - F::cast_from(0.023994444444444443_f64) * t2431 + F::cast_from(0.07198333333333333_f64) * t2435 - F::cast_from(0.035991666666666665_f64) * t2439 + t1263 + F::cast_from(0.008888888888888889_f64) * t2087 - F::cast_from(0.0022222222222222222_f64) * t25 * t2488 + F::cast_from(0.013333333333333334_f64) * t25 * t2491 - F::cast_from(0.006666666666666667_f64) * t25 * t2494;
    (t2482, t2488, t2491, t2494, t2497)
}
