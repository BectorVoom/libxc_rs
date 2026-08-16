//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 747/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk747(t32435: f64, t7290: f64, t2958: f64, t7291: f64, t123: f64, t24884: f64, t10627: f64, t1858: f64, t787: f64, t1980: f64, t8792: f64, t1: f64, t106: f64, t5745: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32436 = t7290 * t32435;
    let t32607 = t2958 * t7291;
    let t32692 = t24884 * t123;
    let t32743 = t1858 * t10627;
    let t32744 = t787 * t32743;
    let t32757 = t1980 * t8792;
    let t32809 = t787 * t5745 * t1 * t106;
    (t32436, t32607, t32692, t32743, t32744, t32757, t32809)
}
