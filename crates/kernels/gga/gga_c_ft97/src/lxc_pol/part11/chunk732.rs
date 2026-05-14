//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 732/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk732<F: Float>(t2253: F, t2930: F, t703: F, t900: F, t2405: F, t904: F, t327: F, t9577: F, t2917: F, t9571: F, t4334: F, t9583: F, t2413: F, t2923: F, t2951: F, t684: F) -> (F, F, F, F, F, F, F, F) {
    let t10843 = t2253 * t2930;
    let t10845 = t703 * t900;
    let t10846 = t2405 * t904;
    let t10847 = t10845 * t10846;
    let t10850 = t327 * t9577;
    let t10852 = t2917 * t10850 * t9571;
    let t10855 = t4334 * t9583;
    let t10858 = t2413 * t904;
    let t10859 = t2923 * t10858;
    let t10861 = t684 * t2951;
    (t10843, t10845, t10847, t10850, t10852, t10855, t10859, t10861)
}
