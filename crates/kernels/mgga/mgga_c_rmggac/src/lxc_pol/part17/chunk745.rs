//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 745/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk745<F: Float>(t40898: F, t7785: F, t321: F, t8708: F, t262: F, t7844: F, t39693: F, t39697: F, t7788: F, t333: F, t8712: F, t7829: F, t26: F, t7834: F, t797: F, t838: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t40899 = t7785 * t40898;
    let t40905 = t8708 * t321;
    let t40906 = t262 * t40905;
    let t40907 = t7844 * t40906;
    let t40908 = 0.10909864661698136691e0 * t40907;
    let t40911 = t7785 * t39693;
    let t40918 = t7788 * t39697;
    let t40920 = t8712 * t333;
    let t40921 = t262 * t40920;
    let t40922 = t7829 * t40921;
    let t40927 = t7834 * t26;
    let t40928 = t797 * t40927;
    let t40932 = t838 * t40927;
    (t40899, t40905, t40906, t40908, t40911, t40918, t40920, t40921, t40922, t40928, t40932)
}
