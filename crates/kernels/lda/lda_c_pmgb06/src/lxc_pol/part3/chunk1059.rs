//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1059/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1059<F: Float>(t13718: F, t13720: F, t13722: F, t13725: F, t13728: F, t13730: F, t13732: F, t13734: F, t13739: F, t13741: F, t13743: F, t13745: F, t13747: F, t13749: F, t13751: F, t13753: F, t13755: F, t13757: F, t13759: F, t13762: F, t13764: F, t13767: F, t13769: F) -> (F, F) {
    let t14439 = t13718 - t13720 + t13722 + t13725 + t13728 + t13730 + t13732 + t13734 + t13739 + t13741 + t13743;
    let t14440 = t13745 + t13747 + t13749 + t13751 + t13753 + t13755 - t13757 - t13759 + t13762 + t13764 + t13767 - t13769;
    (t14439, t14440)
}
