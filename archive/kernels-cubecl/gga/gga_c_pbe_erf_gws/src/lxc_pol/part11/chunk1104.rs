//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1104/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1104<F: Float>(t31225: F, t12647: F, t2612: F, t1017: F, t1820: F, t1885: F, t40676: F, t16845: F, t18196: F, t47675: F, t47676: F, t47677: F, t47678: F, t47679: F, t47683: F, t47684: F) -> (F, F, F, F) {
    let t47685 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t31225;
    let t47687 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t2612 * t12647;
    let t47691 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1820 * t1885 * t40676 * t1017;
    let t47692 = t47675 + t18196 + t47676 + t47677 - t47678 + t47679 - t47683 - t16845 - t47684 + t47685 + t47687 - t47691;
    (t47685, t47687, t47691, t47692)
}
