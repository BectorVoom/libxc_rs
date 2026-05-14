//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 960/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk960<F: Float>(t1882: F, t9831: F, t10123: F, t3281: F, t768: F, t2559: F, t8232: F, t2563: F, t731: F, t10092: F, t1901: F, t1934: F, t2405: F, t2526: F, t2606: F, t2607: F, t3885: F, t3891: F, t42399: F, t766: F, t8608: F, t9787: F, t9854: F) -> (F,) {
    let t42606 = t1882 * t9831;
    let t42608 = t1882 * t10123;
    let t42610 = t3281 * t768;
    let t42612 = t8232 * t2559;
    let t42614 = t8232 * t2563;
    let t42616 = t3281 * t731;
    let t42639 = 8.0 / 9.0 * t42606 + 4.0 / 9.0 * t42608 + 112.0 / 81.0 * t42610 - 16.0 / 9.0 * t42612 - 8.0 / 9.0 * t42614 + 112.0 / 81.0 * t42616 + 4.0 / 9.0 * t1901 * t3891 * t10092 * t2405 + 4.0 / 9.0 * t1901 * t2606 * t2607 * t8608 * t766 + 2.0 / 3.0 * t1901 * t2606 * t2607 * t1934 * t2526 + 8.0 / 3.0 * t1901 * t9787 * t9854 + 8.0 / 9.0 * t1901 * t2606 * t3885 * t42399;
    (t42639,)
}
