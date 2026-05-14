//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 756/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk756<F: Float>(t6205: F, t826: F, t6198: F, t799: F, t2532: F, t4763: F, t6188: F, t811: F, t1466: F, t1318: F, t4734: F, t7545: F, t7547: F, t7548: F, t7549: F, t7550: F, t7551: F, t7552: F, t7553: F, t7554: F, t7556: F, t7560: F, t7562: F) -> (F, F, F, F, F, F, F) {
    let t7564 = 4.0 / 15.0 * t6205 * t826;
    let t7566 = 4.0 / 15.0 * t6198 * t799;
    let t7568 = 8.0 / 5.0 * t4763 * t2532;
    let t7569 = t6188 * t811;
    let t7570 = t1466 * t7569;
    let t7572 = 4.0 / 5.0 * t1318 * t7570;
    let t7573 = t7545 + 8.0 * t4734 - t7547 - t7548 - t7549 + t7550 + t7551 + t7552 + t7553 + t7554 + t7556 + t7560 + t7562 + t7564 + t7566 - t7568 - t7572;
    (t7564, t7566, t7568, t7569, t7570, t7572, t7573)
}
