//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 679/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk679<F: Float>(t500: F, t6134: F, t1972: F, t1989: F, t2855: F, t2858: F, t2854: F, t4593: F, t4624: F, t4717: F, t6111: F, t6116: F, t6118: F, t6122: F, t6126: F, t6129: F, t6133: F) -> (F, F, F, F, F) {
    let t6136 = t6134 * t500 / F::cast_from(45.0_f64);
    let t6138 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1972 * t1989;
    let t6139 = t2855 / F::cast_from(135.0_f64);
    let t6140 = t2858 / F::cast_from(135.0_f64);
    let t6141 = t6111 + t6116 + t6118 + t6122 + t6126 + t6129 + t6133 + t6136 + t6138 + t2854 - t6139 - t6140 + t4593 + t4624 - t4717;
    (t6136, t6138, t6139, t6140, t6141)
}
