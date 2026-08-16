//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1155/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1155(t10389: f64, t10391: f64, t10483: f64, t10486: f64, t10488: f64, t10490: f64, t42554: f64, t42555: f64, t9060: f64, t9062: f64, t9075: f64, t10501: f64, t10502: f64, t10506: f64, t10507: f64, t42559: f64, t42560: f64, t42561: f64, t42562: f64, t42563: f64, t9083: f64, t9091: f64) -> (f64, f64) {
    let t49882 = -0.95793933614910468512e0_f64 * t9060 + 0.63862622409940312341e0_f64 * t9062 - t10389 - t10391 + t10483 - t10486 - t42554 - t42555 + 0.3193131120497015617e0_f64 * t9075 + t10488 + t10490;
    let t49888 = -t10501 - t10502 + 0.2881692658299671676e-2_f64 * t9083 + t42559 - 0.79453919800822633545e-4_f64 * t9091 + t42560 - t42561 - t42562 - t10506 - t10507 + t42563;
    (t49882, t49888)
}
