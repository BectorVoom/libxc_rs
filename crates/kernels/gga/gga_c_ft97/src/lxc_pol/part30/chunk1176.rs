//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1176/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1176<F: Float>(t1466: F, t36104: F, t681: F, t36096: F, t28960: F, t7581: F, t111668: F, t142918: F, t142925: F, t153372: F, t193: F, t28966: F, t28972: F, t29017: F, t29033: F, t317: F, t33983: F, t4309: F, t6222: F, t7612: F, t798: F) -> F {
    let t154911 = t1466 * t681 * t36104;
    let t154914 = t1466 * t681 * t36096;
    let t154941 = t7581 * t28960;
    let t154945 = -t154911 / F::new(18.0) - t154914 / F::new(9.0) - t1466 * t193 * t33983 * t29033 / F::new(3.0) - t1466 * t193 * t33983 * t28966 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1466 * t193 * t6222 * t111668 + t1466 * t193 * t7612 * t4309 / F::new(6.0) + t7581 * t29017 / F::new(6.0) - t142918 / F::new(18.0) - t142925 / F::new(9.0) + t1466 * t193 * t798 * t153372 * t317 / F::new(6.0) - t154941 / F::new(18.0) - t7581 * t28972 / F::new(3.0);
    t154945
}
