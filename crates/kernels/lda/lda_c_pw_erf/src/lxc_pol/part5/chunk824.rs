//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 824/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk824<F: Float>(t1466: F, t7557: F, t571: F, t7007: F, t826: F, t6205: F, t6198: F, t799: F, t2532: F, t4763: F, t6188: F, t811: F) -> (F, F, F, F, F, F, F) {
    let t7558 = t1466 * t7557;
    let t7560 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t571 * t7558;
    let t7562 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t7007 * t826;
    let t7564 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t6205 * t826;
    let t7566 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t6198 * t799;
    let t7568 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4763 * t2532;
    let t7569 = t6188 * t811;
    (t7558, t7560, t7562, t7564, t7566, t7568, t7569)
}
