//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1342/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1342<F: Float>(t13672: F, t17466: F, t5069: F, t12546: F, t17070: F, t17593: F, t17597: F, t17601: F, t17604: F, t17607: F, t17610: F, t17614: F, t17616: F, t17620: F, t17624: F, t17627: F, t17631: F, t17634: F) -> (F, F, F) {
    let t17637 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t13672 * t5069 * t17466;
    let t17640 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t13672 * t12546 * t17070;
    let t17641 = t17593 - t17597 + t17601 + t17604 - t17607 + t17610 + t17614 + t17616 + t17620 - t17624 + t17627 - t17631 + t17634 - t17637 - t17640;
    (t17637, t17640, t17641)
}
