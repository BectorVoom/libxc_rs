//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1182/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1182<F: Float>(t15926: F, t6993: F, t581: F, t7456: F, t1318: F, t1466: F, t549: F, t15619: F, t571: F, t833: F, t1440: F, t2098: F, t519: F, t7002: F) -> (F, F, F, F) {
    let t21500 = F::new(4.0) / F::new(5.0) * t15926 * t6993;
    let t21501 = t581 * t7456;
    let t21505 = F::new(4.0) / F::new(15.0) * t1318 * t1466 * t21501 * t549;
    let t21509 = F::new(4.0) / F::new(5.0) * t571 * t1466 * t15619 * t833;
    let t21513 = F::new(4.0) / F::new(5.0) * t519 * t1440 * t7002 * t2098;
    (t21500, t21505, t21509, t21513)
}
