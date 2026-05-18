//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 831/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk831<F: Float>(t1217: F, t858: F, t34: F, t92: F, t93: F, t108: F, t2268: F, t2271: F, t2274: F, t2277: F, t39: F, t4356: F, t4371: F, t462: F, t659: F, t661: F, t753: F, t754: F, t940: F, t945: F, t951: F, t954: F) -> (F, F, F, F) {
    let t5806 = t858 * t1217;
    let t5812 = t92 * t34;
    let t5823 = t93 * t34;
    let t5833 = (F::new(40.0) / F::new(27.0) * t753 * t940 + F::new(80.0) / F::new(9.0) * t5812 * t4356 + F::new(20.0) / F::new(9.0) * t2268 * t945 + F::new(8.0) / F::new(3.0) * t659 * t462 - F::new(8.0) * t2271 * t39 + F::new(40.0) / F::new(27.0) * t754 * t951 - F::new(80.0) / F::new(9.0) * t5823 * t4371 + F::new(20.0) / F::new(9.0) * t2274 * t954 - F::new(8.0) / F::new(3.0) * t661 * t462 + F::new(8.0) * t2277 * t39) * t108;
    (t5806, t5812, t5823, t5833)
}
