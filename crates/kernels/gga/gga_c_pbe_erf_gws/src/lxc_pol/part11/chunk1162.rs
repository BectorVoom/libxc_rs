//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1162/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1162<F: Float>(t50: F, t12355: F, t1412: F, t18684: F, t2465: F, t3354: F, t47372: F, t47377: F, t47733: F, t52: F, t9993: F, t48458: F, t59: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t48470 = piecewise3::<F>(t51, F::new(0.0), F::new(40.0) / F::new(81.0) * t18684 * t47377 - F::new(16.0) / F::new(9.0) * t9993 * t3354 + F::new(4.0) / F::new(3.0) * t1412 * t47733 + F::new(16.0) / F::new(9.0) * t2465 * t12355 + F::new(4.0) / F::new(3.0) * t52 * t47372);
    let t48472 = (t48458 + t48470) * t59;
    t48472
}
