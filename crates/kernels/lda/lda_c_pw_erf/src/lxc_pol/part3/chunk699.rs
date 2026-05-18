//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 699/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk699<F: Float>(t50: F, t34: F, t950: F, t352: F, t462: F, t1789: F, t1792: F, t39: F, t4367: F, t52: F, t951: F, t954: F, t4366: F, t59: F, zeta_threshold: F) -> (F, F, F) {
    let t51 = t50 <= zeta_threshold;
    let t4370 = t950 * t34;
    let t4371 = t462 * t352;
    let t4381 = piecewise3::<f64>(t51, F::new(0.0), -F::new(8.0) / F::new(27.0) * t4367 * t951 - F::new(16.0) / F::new(9.0) * t4370 * t4371 + F::new(4.0) / F::new(9.0) * t1789 * t954 - F::new(8.0) / F::new(3.0) * t52 * t462 + F::new(8.0) * t1792 * t39);
    let t4383 = (t4366 + t4381) * t59;
    (t4370, t4371, t4383)
}
