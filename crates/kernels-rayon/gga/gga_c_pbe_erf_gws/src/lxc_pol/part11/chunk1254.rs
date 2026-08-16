//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1254/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1254(t13086: f64, t343: f64, t2168: f64, t2170: f64, t3131: f64, t11412: f64, t13523: f64, t2253: f64, t2306: f64, t3257: f64, t3747: f64, t37645: f64, t3803: f64, t45741: f64, t49730: f64, t49894: f64, t49895: f64, t49899: f64, t49903: f64, t49907: f64, t6275: f64, t6637: f64, t9499: f64, t9847: f64) -> (f64, f64, f64) {
    let t49908 = t343 * t13086;
    let t49912 = t2168 * t2170 * t3131 * t49908 / 12.0_f64;
    let t49919 = t6275 * t9499 * t3747 * t11412 / 16.0_f64 + t6637 * t9499 * t9847 * t13523 / 96.0_f64 + t6275 * t37645 * t2306 * t11412 / 8.0_f64 + t49894 + t49895 - t49899 + t49903 + t49907 + t49912 + 7.0_f64 / 576.0_f64 * t45741 - t2253 * t3257 * t3803 * t49730 * t343 / 64.0_f64;
    (t49908, t49912, t49919)
}
