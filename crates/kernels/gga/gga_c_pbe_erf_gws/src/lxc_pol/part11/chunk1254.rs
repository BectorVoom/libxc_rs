//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1254/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1254<F: Float>(t13086: F, t343: F, t2168: F, t2170: F, t3131: F, t11412: F, t13523: F, t2253: F, t2306: F, t3257: F, t3747: F, t37645: F, t3803: F, t45741: F, t49730: F, t49894: F, t49895: F, t49899: F, t49903: F, t49907: F, t6275: F, t6637: F, t9499: F, t9847: F) -> (F, F, F) {
    let t49908 = t343 * t13086;
    let t49912 = t2168 * t2170 * t3131 * t49908 / F::cast_from(12.0_f64);
    let t49919 = t6275 * t9499 * t3747 * t11412 / F::cast_from(16.0_f64) + t6637 * t9499 * t9847 * t13523 / F::cast_from(96.0_f64) + t6275 * t37645 * t2306 * t11412 / F::cast_from(8.0_f64) + t49894 + t49895 - t49899 + t49903 + t49907 + t49912 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t45741 - t2253 * t3257 * t3803 * t49730 * t343 / F::cast_from(64.0_f64);
    (t49908, t49912, t49919)
}
