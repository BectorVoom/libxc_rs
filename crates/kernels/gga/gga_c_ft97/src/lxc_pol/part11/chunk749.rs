//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 749/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk749<F: Float>(t10478: F, t319: F, t2766: F, t871: F, t10491: F, t2843: F, t848: F, t863: F, t2681: F, t309: F, t10580: F, t312: F, t9570: F, t9577: F, t799: F, t339: F, t39: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15290 = t10478 * t319;
    let t15294 = t2766 * t871;
    let t15299 = t10491 * t319;
    let t15312 = t848 * t2843;
    let t15365 = t2766 * t863;
    let t15369 = t2681 * t309;
    let t15385 = t10580 * t309;
    let t15386 = t312 * t9570;
    let t15402 = t312 * t9577;
    let t15460 = t799 * t309;
    let t15564 = t339 * t39;
    (t15290, t15294, t15299, t15312, t15365, t15369, t15385, t15386, t15402, t15460, t15564)
}
