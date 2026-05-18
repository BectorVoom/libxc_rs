//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1350/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1350<F: Float>(t12555: F, t6639: F, t6643: F, t12516: F, t6646: F, t10316: F, t12684: F, t13043: F, t13296: F, t13770: F, t13775: F, t13782: F, t13807: F, t1629: F, t16829: F, t17483: F, t3290: F, t443: F, t477: F, t5077: F, t6636: F, t6637: F, t6638: F, t6642: F) -> F {
    let t17734 = t12555 * t6639;
    let t17736 = t12555 * t6643;
    let t17738 = t12516 * t6646;
    let t17765 = -F::new(8.0) / F::new(27.0) * t13770 - F::new(4.0) / F::new(81.0) * t13775 - F::new(4.0) / F::new(135.0) * t13782 + F::new(16.0) / F::new(135.0) * t17734 + F::new(16.0) / F::new(135.0) * t17736 - F::new(8.0) / F::new(81.0) * t17738 + F::new(8.0) / F::new(45.0) * t12684 * t6639 + F::new(8.0) / F::new(45.0) * t5077 * t3290 * t443 * t6638 + F::new(8.0) / F::new(45.0) * t5077 * t6636 * t17483 * t477 + F::new(4.0) / F::new(45.0) * t5077 * t6636 * t6637 * t1629 + F::new(16.0) / F::new(45.0) * t13043 * t6636 * t16829 * t477 + F::new(8.0) / F::new(45.0) * t12684 * t6643 + F::new(8.0) / F::new(45.0) * t5077 * t13296 * t6642 + F::new(8.0) / F::new(45.0) * t13807 - F::new(4.0) / F::new(405.0) * t10316;
    t17765
}
