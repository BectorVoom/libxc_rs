//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1178/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1178<F: Float>(t132: F, t435: F, t7802: F, t14212: F, t17982: F, t17984: F, t17991: F, t17993: F, t17996: F, t18002: F, t18004: F, t18006: F, t18008: F, t18010: F) -> F {
    let t21242 = t132 * t435 * t7802;
    let t21254 = -t21242 / F::new(45.0) + t14212 + F::new(2.0) / F::new(27.0) * t17982 + F::new(2.0) / F::new(27.0) * t17984 + F::new(4.0) / F::new(45.0) * t17991 + F::new(4.0) / F::new(45.0) * t17993 + F::new(8.0) / F::new(45.0) * t17996 + F::new(2.0) / F::new(45.0) * t18002 + F::new(2.0) / F::new(45.0) * t18004 + F::new(2.0) / F::new(45.0) * t18006 - F::new(4.0) / F::new(45.0) * t18008 - F::new(4.0) / F::new(45.0) * t18010;
    t21254
}
