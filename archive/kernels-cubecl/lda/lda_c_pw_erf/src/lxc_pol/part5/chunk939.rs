//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 939/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk939<F: Float>(t1765: F, t2993: F, t2998: F, t2710: F, t4397: F, t2707: F, t2698: F, t2701: F, t1085: F, t1798: F, t4: F, t1769: F, t4295: F) -> (F, F, F, F, F, F, F, F) {
    let t11307 = t1765 * t2993;
    let t11309 = t1765 * t2998;
    let t11313 = t4397 * t2710;
    let t11315 = t4397 * t2707;
    let t11317 = t4397 * t2698;
    let t11319 = t4397 * t2701;
    let t11322 = t1798 * t4 * t1085;
    let t11323 = F::cast_from(0.032530742648344574_f64) * t11322;
    let t11325 = t1769 * t4295;
    (t11307, t11309, t11313, t11315, t11317, t11319, t11323, t11325)
}
