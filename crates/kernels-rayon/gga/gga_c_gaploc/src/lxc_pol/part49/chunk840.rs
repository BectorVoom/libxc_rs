//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 840/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk840(t10627: f64, t1858: f64, t787: f64, t1980: f64, t8792: f64, t1: f64, t106: f64, t5745: f64, t191: f64, t5750: f64, t2925: f64, t5241: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32743 = t1858 * t10627;
    let t32744 = t787 * t32743;
    let t32757 = t1980 * t8792;
    let t32809 = t787 * t5745 * t1 * t106;
    let t32810 = t191 * t5750;
    let t32840 = t5241 * t2925;
    (t32743, t32744, t32757, t32809, t32810, t32840)
}
