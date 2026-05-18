//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 872/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk872<F: Float>(t1003: F, t1004: F, t1008: F, t1009: F, t1011: F, t1054: F, t1061: F, t350: F, t3725: F, t3729: F, t3793: F, t3803: F, t666: F, t667: F, t682: F, t8482: F, t8519: F, t8522: F, t8552: F, t8594: F, t8598: F, t8599: F, t8610: F, t8621: F, t8863: F, t8867: F, t967: F, t991: F, t992: F) -> F {
    let t8887 = -t8482 + t8519 - F::new(3.5089341735807875) * t1054 * t8522 * t682 + F::new(51.94757731704439) * t1061 * t8522 * t967 + F::new(623.3709278045327) * t3803 * t8599 * t967 + F::new(96.49187699215521) * t1009 * t8863 * t1011 - F::new(24.0) * t3793 * t8867 * t667 - F::new(6.0) * t992 * t8863 * t667 + t8552 + t8594 + t8598 - t8610 - F::new(6.609050294782684) * t350 * t1008 * t1003 * t1011 * t666 + F::new(0.41096) * t350 * t991 * t666 * t1004 + F::new(0.13012297560362088) * t350 * t3729 - F::new(1.9263893255070628) * t350 * t3725 - t8621;
    t8887
}
