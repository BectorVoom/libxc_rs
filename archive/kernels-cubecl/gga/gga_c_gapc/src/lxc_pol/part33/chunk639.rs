//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 639/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk639<F: Float>(t1096: F, t2469: F, t3265: F, t338: F, t3656: F, t3658: F, t3661: F, t3722: F, t3742: F, t3746: F, t3795: F, t884: F) -> F {
    let t3797 = -F::cast_from(2.0_f64) * t1096 * t3265 + F::cast_from(2.0_f64) * t2469 * t3746 + t338 * t3742 - t3795 * t884 - t3656 + t3658 - t3661 + t3722;
    t3797
}
