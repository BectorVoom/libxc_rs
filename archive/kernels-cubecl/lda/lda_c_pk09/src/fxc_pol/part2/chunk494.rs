//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 494/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk494<F: Float>(t2730: F, t501: F, t507: F, t1667: F, t1670: F, t1674: F, t1677: F, t1679: F, t1748: F, t2740: F, t2745: F, t2749: F, t2753: F, t2755: F, t2759: F, t455: F, t516: F) -> (F, F, F) {
    let t2762 = t501 * t2730;
    let t2765 = t507 * t2730;
    let t2768 = t1667 + t1670 - F::cast_from(2.9824072957409817_f64) * t2740 * t1748 - t1674 + t1677 - t1679 - F::cast_from(5.40024514194619_f64) * t2745 + F::cast_from(5.40024514194619_f64) * t2749 + F::cast_from(22.07984838129906_f64) * t2753 + F::cast_from(18.635258017632964_f64) * t2755 * t455 - F::cast_from(0.04115066352984959_f64) * t2759 * t516 + F::cast_from(19.489173774580152_f64) * t2762 * t455 + F::cast_from(4.937333717448355_f64) * t2765 * t455;
    (t2762, t2765, t2768)
}
