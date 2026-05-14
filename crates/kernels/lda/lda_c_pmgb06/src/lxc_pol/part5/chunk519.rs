//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 519/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk519<F: Float>(t1138: F, t1145: F, t1151: F, t1753: F, t1765: F, t1772: F, t1775: F, t2172: F, t2176: F, t2267: F, t2269: F, t2747: F, t749: F, t754: F, t97: F, t1786: F, t27: F, t321: F) -> (F, F, F) {
    let t2752 = -0.02394846802050922 * t2267 - 0.0005811348303577384 * t2176 + 0.039914113367515366 * t2269 - 0.10809180959278285 * t2172 + t1138 - t1145 + t1151 + t1753 - t1765 - t1772 - t1775;
    let t2753 = t2747 + t2752;
    let t2760 = t749 * t754 * t97;
    let t2764 = t321 * t1786 * t27;
    (t2753, t2760, t2764)
}
