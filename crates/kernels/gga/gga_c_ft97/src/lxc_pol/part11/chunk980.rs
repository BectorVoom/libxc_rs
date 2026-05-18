//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 980/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk980<F: Float>(t2109: F, t8282: F, t2098: F, t1775: F, t9233: F, t24: F, t38534: F, t38550: F, t38566: F, t40262: F, t40267: F, t40368: F, t40370: F, t40375: F, t40377: F, t40379: F, t40384: F, t40392: F, t462: F, t582: F, t586: F, t92: F, t9224: F) -> F {
    let t40397 = t8282 * t2109;
    let t40399 = t8282 * t2098;
    let t40401 = t1775 * t9233;
    let t40403 = -F::new(8.0) / F::new(3.0) * t40368 + F::new(112.0) / F::new(27.0) * t40370 - t92 * t24 * t586 * t40262 + F::new(4.0) / F::new(3.0) * t40375 + F::new(16.0) / F::new(3.0) * t40377 + F::new(24.0) * t92 * t24 * t40379 * t40267 + F::new(8.0) * t40384 + F::new(2.0) * t462 * t582 * t38566 - t462 * t582 * t38534 / F::new(3.0) - F::new(16.0) / F::new(9.0) * t40392 + F::new(40.0) / F::new(9.0) * t462 * t9224 * t38550 - F::new(8.0) / F::new(9.0) * t40397 - F::new(16.0) / F::new(27.0) * t40399 + F::new(4.0) / F::new(9.0) * t40401;
    t40403
}
