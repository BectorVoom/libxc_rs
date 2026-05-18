//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1165/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1165<F: Float>(t10445: F, t493: F, t529: F, t7594: F, t2002: F, t6416: F, t6419: F, t6465: F, t6475: F, t6275: F, t6478: F, t20981: F, t20984: F, t20987: F, t20992: F, t20995: F) -> (F, F, F, F, F, F, F) {
    let t20999 = F::new(8.0) / F::new(81.0) * t493 * t10445 * t7594 * t529;
    let t21001 = F::new(2.0) / F::new(15.0) * t2002 * t6416;
    let t21003 = t2002 * t6419 / F::new(9.0);
    let t21005 = t2002 * t6465 / F::new(9.0);
    let t21007 = F::new(8.0) / F::new(27.0) * t2002 * t6475;
    let t21009 = F::new(4.0) / F::new(9.0) * t6275 * t6478;
    let t21010 = -t20981 - t20984 - t20987 - t20992 - t20995 - t20999 - t21001 + t21003 - t21005 + t21007 + t21009;
    (t20999, t21001, t21003, t21005, t21007, t21009, t21010)
}
