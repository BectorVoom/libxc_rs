//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 857/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk857<F: Float>(t138: F, t1711: F, t1861: F, t2634: F, t2642: F, t3339: F, t444: F, t5621: F, t7168: F, t774: F, t7949: F, t7957: F, t7960: F, t7974: F) -> F {
    let t7976 = t7949 * t138 + F::new(6.0) * t1711 * t7960 - F::new(3.0) * t1861 * t2642 + F::new(6.0) * t5621 * t2634 - F::new(6.0) * t3339 * t7957 - t444 * t7974 - F::new(3.0) * t7168 * t774;
    t7976
}
