//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1354/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1354<F: Float>(t11398: F, t3959: F, t11757: F, t3972: F, t3975: F, t11588: F, t14617: F, t53688: F, t15195: F, t9270: F, t1109: F, t13925: F, t14397: F, t14420: F, t14437: F, t22379: F, t2409: F, t3066: F, t3067: F, t3306: F, t35000: F, t35260: F, t353: F, t4002: F, t4053: F, t4182: F, t53852: F, t53874: F, t53897: F, t55408: F, t859: F, t8629: F, t8654: F) -> F {
    let t57265 = t3959 * t11398;
    let t57284 = t3972 * t3975 * t11757;
    let t57287 = t3972 * t3975 * t11588;
    let t57289 = t53688 * t14617;
    let t57291 = t9270 * t15195;
    let t57298 = t57265 / F::new(48.0) - F::new(35.0) / F::new(216.0) * t53852 + t35000 * t13925 / F::new(48.0) + t8629 * t859 * t353 * t4053 * t1109 / F::new(96.0) + t22379 * t14420 / F::new(24.0) - t35260 * t4002 / F::new(96.0) - t8654 * t14397 / F::new(48.0) - t8654 * t14437 / F::new(48.0) + t53874 + t55408 + t57284 / F::new(1536.0) + t57287 / F::new(1536.0) - t57289 / F::new(48.0) - t53897 - F::new(7.0) / F::new(72.0) * t57291 + t3066 * t2409 * t3067 * t4182 * t3306 / F::new(24.0);
    t57298
}
