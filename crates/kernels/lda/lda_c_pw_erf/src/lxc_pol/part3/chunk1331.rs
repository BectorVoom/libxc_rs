//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1331/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1331<F: Float>(t5686: F, t5688: F, t11332: F, t11350: F, t11367: F, t11384: F, t11467: F, t15290: F, t15295: F, t15298: F, t2951: F, t2981: F, t4289: F, t4292: F, t4410: F, t5945: F, t7: F, t7333: F, t7334: F, t7335: F, t8106: F, t8107: F, t8108: F, t8109: F, t8110: F, t8113: F, t8114: F) -> F {
    let t15306 = F::new(6.0) * t5686;
    let t15307 = F::new(24.0) * t5688;
    let t15308 = F::cast_from(0.05925536910769562_f64) * t4410 + t7333 - t7334 + t7335 - t8106 + t8107 - t8108 + t8109 + t7 * (t11332 + t11350 + t11367 + t11384 + t11467 + t15290 + t15295 + t15298) - t8110 - t4289 - t8113 + t8114 + F::cast_from(10.526802115419367_f64) * t2951 - F::cast_from(5.694518669548362_f64) * t4292 + t2981 - F::new(3.0) * t5945 + t15306 - t15307;
    t15308
}
