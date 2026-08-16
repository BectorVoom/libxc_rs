//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 825/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk825<F: Float>(t1466: F, t7569: F, t1318: F, t4734: F, t7545: F, t7547: F, t7548: F, t7549: F, t7550: F, t7551: F, t7552: F, t7553: F, t7554: F, t7556: F, t7560: F, t7562: F, t7564: F, t7566: F, t7568: F) -> (F, F, F) {
    let t7570 = t1466 * t7569;
    let t7572 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1318 * t7570;
    let t7573 = t7545 + F::cast_from(8.0_f64) * t4734 - t7547 - t7548 - t7549 + t7550 + t7551 + t7552 + t7553 + t7554 + t7556 + t7560 + t7562 + t7564 + t7566 - t7568 - t7572;
    (t7570, t7572, t7573)
}
