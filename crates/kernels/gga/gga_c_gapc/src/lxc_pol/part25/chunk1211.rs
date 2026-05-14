//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1211/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1211<F: Float>(t33353: F, t33375: F, t33377: F, t33380: F, t36559: F, t36560: F, t36561: F, t36562: F, t36563: F, t36564: F, t36568: F, t33407: F, t36570: F, t36571: F, t36572: F, t36573: F, t36574: F, t36575: F, t36577: F, t36578: F, t36579: F, t36580: F) -> (F, F) {
    let t38726 = 0.90579542097823505428e-7 * t33353 + t36559 + t36560 - t36561 + t36562 + t36563 + t36564 - 0.67632724766374884054e-5 * t33375 - 0.54347725258694103258e-6 * t33377 - 0.18115908419564701086e-6 * t33380 - t36568;
    let t38728 = -t36570 + t36571 - t36572 - t36573 + t36574 - t36575 - 0.36231816839129402172e-6 * t33407 + t36577 + t36578 + t36579 - t36580;
    (t38726, t38728)
}
