//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 386/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk386<F: Float>(t43: F, t50: F, t1775: F, t40: F, t339: F, t749: F, t344: F, t739: F, t939: F, t34: F, t47: F, t348: F, t462: F, t743: F, t950: F, t52: F, t352: F, t59: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1776 = t40 * t1775;
    let t1777 = t339 * t749;
    let t1778 = 4.0 * t1777;
    let t1779 = t344 * t749;
    let t1780 = 4.0 * t1779;
    let t1781 = t939 * t739;
    let t1784 = t47 * t34;
    let t1788 = piecewise3(t44, 0.0, 4.0 / 9.0 * t1781 * t348 + 8.0 / 3.0 * t1784 * t462);
    let t1789 = t950 * t743;
    let t1792 = t52 * t34;
    let t1796 = piecewise3(t51, 0.0, 4.0 / 9.0 * t1789 * t352 - 8.0 / 3.0 * t1792 * t462);
    let t1798 = (t1788 + t1796) * t59;
    (t1776, t1777, t1778, t1779, t1780, t1781, t1789, t1798)
}
