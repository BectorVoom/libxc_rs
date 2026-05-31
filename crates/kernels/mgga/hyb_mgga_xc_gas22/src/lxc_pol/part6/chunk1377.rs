//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1377/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1377<F: Float>(t29821: F, t29848: F, t29882: F, t29911: F, t21393: F, t21396: F, t21427: F, t21430: F, t21433: F, t21638: F, t21641: F, t25214: F, t25217: F, t25220: F, t29819: F) -> (F, F) {
    let t29913 = t29821 + t29848 + t29882 + t29911;
    let t29932 = t21638 - F::cast_from(0.32136222222222222222e1_f64) * t21393 + F::cast_from(0.68863333333333333333e0_f64) * t21396 + t21641 + F::cast_from(0.34731666666666666666e0_f64) * t21430 - F::cast_from(0.18523555555555555555e1_f64) * t21427 + F::cast_from(0.34731666666666666666e0_f64) * t21433 - F::cast_from(0.32136222222222222223e1_f64) * t25214 + F::cast_from(0.27545333333333333334e1_f64) * t25217 - F::cast_from(0.103295e1_f64) * t25220 + F::cast_from(0.3529725e1_f64) * t29819;
    (t29913, t29932)
}
