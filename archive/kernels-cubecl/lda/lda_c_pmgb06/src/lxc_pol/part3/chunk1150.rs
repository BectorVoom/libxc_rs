//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1150/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1150<F: Float>(t13727: F, t1427: F, t5220: F, t1431: F, t1441: F, t13707: F, t13709: F, t13711: F, t13714: F, t13718: F, t13720: F, t13722: F, t13725: F) -> (F, F, F, F, F) {
    let t13728 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13727;
    let t13729 = t5220 * t1427;
    let t13730 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13729;
    let t13731 = t5220 * t1431;
    let t13732 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t13731;
    let t13733 = t5220 * t1441;
    let t13734 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t13733;
    let t13735 = t13707 - t13709 - t13711 - t13714 + t13718 - t13720 + t13722 + t13725 + t13728 + t13730 + t13732 + t13734;
    (t13728, t13730, t13732, t13734, t13735)
}
