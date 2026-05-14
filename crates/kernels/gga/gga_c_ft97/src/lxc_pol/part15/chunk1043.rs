//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1043/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1043<F: Float>(t90516: F, t90537: F, t1212: F, t21181: F, t1248: F, t15229: F, t15254: F, t15369: F, t15370: F, t15385: F, t15386: F, t1901: F, t22208: F, t22391: F, t2881: F, t3699: F, t4139: F, t4140: F, t4265: F, t44335: F, t44340: F, t70000: F, t82630: F, t82638: F, t88105: F, t89212: F, t89813: F) -> (F, F, F, F) {
    let t90538 = t90516 + t90537;
    let t90558 = t21181 * t1212;
    let t90603 = t21181 * t1248;
    let t90620 = 8.0 / 9.0 * t1901 * t2881 * t4265 * t89212 - 8.0 / 27.0 * t1901 * t4139 * t4140 * t89212 - 8.0 * t1901 * t15369 * t15370 * t22208 + 4.0 / 9.0 * t82630 + 4.0 / 3.0 * t82638 + 40.0 / 81.0 * t1901 * t44335 * t15386 * t90558 + 40.0 / 81.0 * t1901 * t15385 * t44340 * t90603 - 20.0 / 27.0 * t1901 * t15385 * t15386 * t88105 - 16.0 / 27.0 * t70000 - 8.0 / 3.0 * t1901 * t15229 * t89813 - 8.0 / 3.0 * t1901 * t15254 * t3699 * t22391;
    (t90538, t90558, t90603, t90620)
}
