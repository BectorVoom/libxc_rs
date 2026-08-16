//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1441/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1441<F: Float>(t15376: F, t15390: F, t18409: F, t18420: F, t18427: F, t18469: F, t22072: F, t22075: F, t22090: F, t22095: F, t3447: F, t4904: F, t4919: F, t52081: F, t64648: F, t73181: F, t73201: F, t73405: F, t73427: F) -> F {
    let t78489 = -F::cast_from(0.22222222222222222222e-2_f64) * t3447 * t64648 * t18469 - F::cast_from(0.88888888888888888887e-2_f64) * t15376 * t22095 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t73201 * t4904 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t4919 * t73405 - F::cast_from(0.88888888888888888886e-2_f64) * t15376 * t22072 - F::cast_from(0.11111111111111111111e-2_f64) * t73427 - F::cast_from(0.17777777777777777777e-1_f64) * t15376 * t22075 - F::cast_from(0.88888888888888888886e-2_f64) * t3447 * t15390 * t73181 + F::cast_from(0.17777777777777777777e-1_f64) * t15376 * t22090 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t18420 * t18409 + F::cast_from(0.33333333333333333332e-2_f64) * t3447 * t18420 * t18427 - F::cast_from(0.12345679012345679012e-2_f64) * t52081;
    t78489
}
