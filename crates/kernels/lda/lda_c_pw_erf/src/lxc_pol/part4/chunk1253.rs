//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1253/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1253<F: Float>(t2566: F, t3745: F, t4738: F, t5378: F, t4953: F, t4959: F, t1486: F, t4867: F, t571: F, t743: F, t13487: F, t799: F, t511: F, t6306: F, t1948: F, t3974: F, t3977: F, t4475: F) -> (F, F, F, F, F, F, F, F) {
    let t18629 = 16.0 / 45.0 * t3745 * t2566;
    let t18630 = t4738 * t5378;
    let t18631 = 32.0 / 45.0 * t18630;
    let t18633 = 16.0 / 15.0 * t4738 * t4953;
    let t18635 = 16.0 / 15.0 * t4738 * t4959;
    let t18639 = 16.0 / 27.0 * t571 * t4867 * t1486 * t743;
    let t18641 = 16.0 / 45.0 * t13487 * t799;
    let t18642 = t511 * t6306;
    let t18643 = 16.0 / 45.0 * t18642;
    let t18647 = 64.0 / 45.0 * t3974 * t4475 * t1948 * t3977;
    (t18629, t18631, t18633, t18635, t18639, t18641, t18643, t18647)
}
