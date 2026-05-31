//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 696/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk696<F: Float>(t1422: F, t986: F, t2457: F, t2505: F, t2545: F, t2550: F, t3461: F, t3472: F, t3486: F, t3491: F, t3497: F, t3499: F, t3503: F, t3507: F, t3511: F) -> (F, F) {
    let t3532 = t1422 * t986;
    let t3546 = -F::cast_from(0.17648625e1_f64) * t3486 + F::cast_from(0.3529725e1_f64) * t3491 + t2545 - F::cast_from(0.516475e0_f64) * t2457 - F::cast_from(0.516475e0_f64) * t3461 + F::cast_from(0.1549425e1_f64) * t3472 + F::cast_from(0.31558125e0_f64) * t3497 + F::cast_from(0.6311625e0_f64) * t3499 + t2550 - F::cast_from(0.20839e0_f64) * t2505 - F::cast_from(0.20839e0_f64) * t3503 + F::cast_from(0.312585e0_f64) * t3507 + F::cast_from(0.312585e0_f64) * t3511;
    (t3532, t3546)
}
