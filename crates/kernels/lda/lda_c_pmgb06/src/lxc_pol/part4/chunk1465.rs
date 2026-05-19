//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1465/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1465<F: Float>(t18744: F, t69: F, t11515: F, t11519: F, t11521: F, t18656: F, t18671: F, t18674: F, t18677: F, t18684: F, t18685: F, t18688: F, t18693: F, t18694: F, t18696: F, t18697: F, t18700: F) -> F {
    let t18837 = t69 * t18744;
    let t18842 = -F::cast_from(0.7663355555555555_f64) * t18837 + F::cast_from(1.1495033333333333_f64) * t11515 - F::cast_from(3.065342222222222_f64) * t11519 + F::cast_from(3.5762325925925924_f64) * t11521 + t18656 - t18671 + t18674 + t18677 + t18684 + t18685 + t18688 - t18693 - t18694 - t18696 - t18697 + t18700;
    t18842
}
