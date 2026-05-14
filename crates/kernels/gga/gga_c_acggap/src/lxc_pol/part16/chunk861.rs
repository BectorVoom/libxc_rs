//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 861/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk861<F: Float>(t33799: F, t7943: F, t315: F, t33428: F, t2134: F, t119: F, t8993: F, t1432: F, t30147: F, t30148: F, t7842: F, t1165: F, t5249: F, t604: F, t7493: F, t7433: F, t8869: F) -> (F, F, F, F, F, F) {
    let t33801 = 0.17347256376410398924e1 * t33799 * t7943;
    let t33802 = t315 * t33428;
    let t33804 = 0.17347256376410398924e1 * t33802 * t2134;
    let t33818 = t119 * t8993;
    let t33831 = t30147 * t7842 * t30148 * t1432;
    let t33839 = t7493 * t1165 * t604 * t5249;
    let t33840 = 0.15724046144802076034e-2 * t33839;
    let t33841 = t7433 * t8869;
    (t33801, t33804, t33818, t33831, t33840, t33841)
}
