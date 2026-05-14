//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 603/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk603<F: Float>(t1096: F, t2469: F, t3265: F, t338: F, t3656: F, t3658: F, t3661: F, t3722: F, t3742: F, t3746: F, t3795: F, t884: F, t125: F, t1458: F, t144: F, t667: F) -> (F, F, F) {
    let t3797 = -2.0 * t1096 * t3265 + 2.0 * t2469 * t3746 + t338 * t3742 - t3795 * t884 - t3656 + t3658 - t3661 + t3722;
    let t3938 = t1458 * t125;
    let t3940 = t667 * t144;
    (t3797, t3938, t3940)
}
