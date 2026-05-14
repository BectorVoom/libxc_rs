//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 876/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk876<F: Float>(t39697: F, t7788: F, t333: F, t8712: F, t262: F, t7829: F, t35960: F, t5145: F, t649: F, t26: F, t7834: F, t797: F, t5149: F, t838: F, t5268: F, t40756: F) -> (F, F, F, F, F, F, F, F) {
    let t40918 = t7788 * t39697;
    let t40920 = t8712 * t333;
    let t40921 = t262 * t40920;
    let t40922 = t7829 * t40921;
    let t40925 = t35960 * t649 * t5145;
    let t40927 = t7834 * t26;
    let t40928 = t797 * t40927;
    let t40930 = t40928 * t649 * t5149;
    let t40932 = t838 * t40927;
    let t40934 = t40932 * t649 * t5268;
    let t40938 = t797 * t40756;
    (t40918, t40920, t40921, t40922, t40925, t40930, t40934, t40938)
}
