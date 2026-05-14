//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1246/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1246<F: Float>(t11400: F, t11402: F, t8300: F, t8306: F, t8328: F, t11395: F, t11398: F, t11405: F, t8313: F, t8324: F, t8339: F, t8346: F, t8353: F, t8355: F, t8370: F, t8374: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18693 = 2.5982444444444446 * t11400;
    let t18694 = 2.5982444444444446 * t11402;
    let t18696 = 0.6495611111111111 * t8300;
    let t18697 = 2.5982444444444446 * t8306;
    let t18700 = 0.3247805555555556 * t8328;
    let t18702 = -3.91744 * t11395 + 3.91744 * t11398 - t18693 - t18694 - 8.0 / 9.0 * t11405 - t18696 - t18697 - 2.0 / 9.0 * t8313 - 0.48968 * t8324 + t18700 - t8339 + t8346 + 4.570346666666667 * t8353;
    let t18704 = 3.031285185185185 * t8355;
    let t18706 = 1.2991222222222223 * t8370;
    let t18707 = 0.6495611111111111 * t8374;
    (t18693, t18694, t18696, t18697, t18700, t18702, t18704, t18706, t18707)
}
