//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1214/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1214<F: Float>(t37765: F, t37770: F, t37788: F, t39686: F, t39689: F, t39692: F, t39695: F, t39697: F, t39700: F, t39703: F, t39706: F, t39708: F) -> F {
    let t41511 = -F::cast_from(0.51220160311720645768e0_f64) * t37765 - F::cast_from(0.21951497276451705328e-1_f64) * t37770 - F::cast_from(0.46230515946956099004e0_f64) * t37788 + F::cast_from(0.13099107994629972538e-1_f64) * t39686 - F::cast_from(0.87327386630866483588e-2_f64) * t39689 - F::cast_from(0.26198215989259945076e-1_f64) * t39692 + F::cast_from(0.1047928639570397803e0_f64) * t39695 - F::cast_from(0.87327386630866483588e-2_f64) * t39697 - F::cast_from(0.87327386630866483588e-2_f64) * t39700 + F::cast_from(0.87327386630866483588e-2_f64) * t39703 - F::cast_from(0.43663693315433241794e-2_f64) * t39706 - F::cast_from(0.87327386630866483588e-2_f64) * t39708;
    t41511
}
