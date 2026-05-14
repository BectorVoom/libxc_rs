//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1097/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1097<F: Float>(t13525: F, t37750: F, t38234: F, t1134: F, t2168: F, t3139: F, t44741: F, t2170: F, t44254: F, t49483: F, t8903: F, t3814: F, t13086: F, t343: F, t3131: F, t11412: F, t13523: F, t2253: F, t2306: F, t3257: F, t3747: F, t37645: F, t3803: F, t45741: F, t49730: F, t6275: F, t6637: F, t9499: F, t9847: F) -> (F, F, F, F, F, F, F, F) {
    let t49894 = t37750 * t13525 / 12.0;
    let t49895 = 35.0 / 72.0 * t38234;
    let t49899 = t2168 * t3139 * t44741 * t1134 / 24.0;
    let t49903 = t8903 * t2170 * t44254 * t49483 / 2.0;
    let t49907 = t2168 * t2170 * t44254 * t3814 / 12.0;
    let t49908 = t343 * t13086;
    let t49912 = t2168 * t2170 * t3131 * t49908 / 12.0;
    let t49919 = t6275 * t9499 * t3747 * t11412 / 16.0 + t6637 * t9499 * t9847 * t13523 / 96.0 + t6275 * t37645 * t2306 * t11412 / 8.0 + t49894 + t49895 - t49899 + t49903 + t49907 + t49912 + 7.0 / 576.0 * t45741 - t2253 * t3257 * t3803 * t49730 * t343 / 64.0;
    (t49894, t49895, t49899, t49903, t49907, t49908, t49912, t49919)
}
