//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 839/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk839<F: Float>(t262: F, t40897: F, t7785: F, t2350: F, t839: F, t35879: F, t321: F, t8708: F, t7844: F, t36250: F, t38565: F, t39693: F, t35824: F, t39045: F, t39697: F, t7788: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t40898 = t262 * t40897;
    let t40899 = t7785 * t40898;
    let t40901 = t2350 * t839;
    let t40902 = t262 * t40901;
    let t40903 = t35879 * t40902;
    let t40905 = t8708 * t321;
    let t40906 = t262 * t40905;
    let t40907 = t7844 * t40906;
    let t40909 = t36250 * t38565;
    let t40911 = t7785 * t39693;
    let t40913 = t35824 * t39045;
    let t40918 = t7788 * t39697;
    (t40898, t40899, t40901, t40902, t40903, t40905, t40906, t40907, t40909, t40911, t40913, t40918)
}
