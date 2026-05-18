//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1152/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1152<F: Float>(t13752: F, t1972: F, t2873: F, t1447: F, t5477: F, t1423: F, t5268: F, t13739: F, t13741: F, t13743: F, t13745: F, t13747: F, t13749: F, t13751: F) -> (F, F, F, F, F) {
    let t13753 = F::new(4.0) / F::new(45.0) * t13752;
    let t13755 = F::new(2.0) / F::new(15.0) * t1972 * t2873;
    let t13756 = t1447 * t5477;
    let t13757 = F::new(4.0) / F::new(45.0) * t13756;
    let t13758 = t1423 * t5268;
    let t13759 = F::new(2.0) / F::new(45.0) * t13758;
    let t13760 = t13739 + t13741 + t13743 + t13745 + t13747 + t13749 + t13751 + t13753 + t13755 - t13757 - t13759;
    (t13753, t13755, t13757, t13759, t13760)
}
