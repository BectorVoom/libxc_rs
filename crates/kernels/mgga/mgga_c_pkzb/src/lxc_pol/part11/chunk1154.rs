//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1154/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1154<F: Float>(t3162: F, t9762: F, t12919: F, t1306: F, t26901: F, t31052: F, t31055: F, t31057: F, t31061: F, t31092: F, t31094: F, t31096: F, t31097: F, t31104: F, t955: F, t22500: F, t9856: F) -> (F, F, F) {
    let t31106 = 0.51947577317044391276e2 * t9762 * t3162;
    let t31107 = 6.0 * t12919 * t1306 * t26901 - t1306 * t31097 * t955 + t31052 - t31055 - t31057 - t31061 - t31092 - t31094 + t31096 - t31104 - t31106;
    let t31109 = 18.0 * t22500 * t9856;
    (t31106, t31107, t31109)
}
