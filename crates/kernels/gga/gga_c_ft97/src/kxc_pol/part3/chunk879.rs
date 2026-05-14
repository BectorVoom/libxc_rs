//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 879/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk879<F: Float>(t10688: F, t5309: F, t296: F, t5330: F, t684: F, t10703: F, t15229: F, t19002: F, t10553: F, t10640: F, t14715: F, t14718: F, t14921: F, t14922: F, t14923: F, t14929: F, t14936: F, t18999: F, t19732: F) -> (F, F, F, F, F) {
    let t19810 = t10688 * t5309;
    let t19811 = t296 * t19810;
    let t19815 = t5330 * t684;
    let t19816 = t10703 * t19815;
    let t19819 = t15229 * t19002;
    let t19826 = -t14921 - t14922 + t14923 - t14929 - t14936 - t10553 + t19732 / 2.0 - t10640 - 8.0 / 27.0 * t14715 - 4.0 / 9.0 * t14718 - 4.0 / 3.0 * t18999;
    (t19810, t19811, t19816, t19819, t19826)
}
