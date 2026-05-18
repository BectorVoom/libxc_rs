//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1244/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1244<F: Float>(t11459: F, t13408: F, t2168: F, t6523: F, t45444: F, t1105: F, t13291: F, t2147: F, t337: F, t9119: F, t3824: F, t816: F) -> (F, F, F, F) {
    let t49717 = F::new(3.0) / F::new(8.0) * t2168 * t6523 * t11459 * t13408;
    let t49722 = F::new(7.0) / F::new(24.0) * t45444;
    let t49729 = t9119 * t2147 * t337 * t13291 * t1105 / F::new(6.0);
    let t49730 = t816 * t3824;
    (t49717, t49722, t49729, t49730)
}
