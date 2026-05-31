//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1349/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1349<F: Float>(t1601: F, t2623: F, t11966: F, t13731: F, t13733: F, t13740: F, t13742: F, t13744: F, t13748: F, t13752: F, t13756: F, t13758: F, t13761: F, t13763: F, t13768: F, t1381: F, t17484: F, t2010: F, t442: F, t5068: F, t5077: F, t5094: F, t822: F) -> F {
    let t17719 = t1601 * t2623;
    let t17729 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t13731 + F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t13733 + F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13740 + F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13742 + F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t13744 + F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t13748 - F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t2010 * t442 * t11966 * t822 + F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5077 * t5094 * t17484 + F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5068 * t17719 * t1381 + F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13752 - F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13756 - F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t13758 + F::cast_from(32.0_f64) / F::cast_from(243.0_f64) * t13761 + F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t13763 - F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13768;
    t17729
}
