//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1193/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1193(t90516: f64, t90537: f64, t1212: f64, t21181: f64, t1248: f64, t15229: f64, t15254: f64, t15369: f64, t15370: f64, t15385: f64, t15386: f64, t1901: f64, t22208: f64, t22391: f64, t2881: f64, t3699: f64, t4139: f64, t4140: f64, t4265: f64, t44335: f64, t44340: f64, t70000: f64, t82630: f64, t82638: f64, t88105: f64, t89212: f64, t89813: f64) -> (f64, f64, f64, f64) {
    let t90538 = t90516 + t90537;
    let t90558 = t21181 * t1212;
    let t90603 = t21181 * t1248;
    let t90620 = 8.0_f64 / 9.0_f64 * t1901 * t2881 * t4265 * t89212 - 8.0_f64 / 27.0_f64 * t1901 * t4139 * t4140 * t89212 - 8.0_f64 * t1901 * t15369 * t15370 * t22208 + 4.0_f64 / 9.0_f64 * t82630 + 4.0_f64 / 3.0_f64 * t82638 + 40.0_f64 / 81.0_f64 * t1901 * t44335 * t15386 * t90558 + 40.0_f64 / 81.0_f64 * t1901 * t15385 * t44340 * t90603 - 20.0_f64 / 27.0_f64 * t1901 * t15385 * t15386 * t88105 - 16.0_f64 / 27.0_f64 * t70000 - 8.0_f64 / 3.0_f64 * t1901 * t15229 * t89813 - 8.0_f64 / 3.0_f64 * t1901 * t15254 * t3699 * t22391;
    (t90538, t90558, t90603, t90620)
}
