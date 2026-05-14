//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 671/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk671<F: Float>(t44: F, t2972: F, t7608: F, t2197: F, t3241: F, t205: F, t2201: F, t568: F, t192: F, t7693: F, t2140: F, t2993: F, t2: F, t619: F, t258: F, t620: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t7741 = t2972 * t7608;
    let t7751 = t3241 * t2197;
    let t7752 = t205 * t7751;
    let t7754 = t568 * t2201;
    let t7755 = t205 * t7754;
    let t7757 = t192 * t7693;
    let t7759 = t2993 * t2140;
    let t7762 = t619 * t2;
    let t7766 = piecewise3(t45, 0.0, -2.0 / 9.0 * t7759 * t620 + 2.0 / 3.0 * t7762 * t258);
    (t7741, t7752, t7755, t7757, t7766)
}
