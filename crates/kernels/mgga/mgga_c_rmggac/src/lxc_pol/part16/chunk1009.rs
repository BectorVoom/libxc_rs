//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1009/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1009<F: Float>(t10389: F, t10391: F, t10483: F, t10486: F, t10488: F, t10490: F, t42554: F, t42555: F, t9060: F, t9062: F, t9075: F, t10501: F, t10502: F, t10506: F, t10507: F, t42559: F, t42560: F, t42561: F, t42562: F, t42563: F, t9083: F, t9091: F) -> (F, F) {
    let t49882 = -0.95793933614910468512e0 * t9060 + 0.63862622409940312341e0 * t9062 - t10389 - t10391 + t10483 - t10486 - t42554 - t42555 + 0.3193131120497015617e0 * t9075 + t10488 + t10490;
    let t49888 = -t10501 - t10502 + 0.2881692658299671676e-2 * t9083 + t42559 - 0.79453919800822633545e-4 * t9091 + t42560 - t42561 - t42562 - t10506 - t10507 + t42563;
    (t49882, t49888)
}
