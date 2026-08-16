//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 721/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk721(t1781: f64, t287: f64, t321: f64, t320: f64, t92: f64, t93: f64, t7592: f64, t7523: f64, t972: f64, t346: f64, t2548: f64, t8: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8229 = t321 * t1781 * t287;
    let t8231 = 0.32196894406625029092e-1_f64 * t320 * t8229;
    let t8285 = t92 * t92;
    let t8287 = 1.0_f64 / t8285 * t93;
    let t8319 = 0.54733333333333333333e-2_f64 * t7592;
    let t8321 = 0.60319259259259259259e1_f64 * t7523;
    let t8343 = t972 * t972;
    let t8344 = 1.0_f64 / t8343;
    let t8345 = t346 * t8344;
    let t8362 = 0.34962962962962962963e3_f64 * t7592;
    let t8364 = 0.22615185185185185185e4_f64 * t7523;
    let t8384 = t8 * t2548;
    (t8231, t8285, t8287, t8319, t8321, t8343, t8344, t8345, t8362, t8364, t8384)
}
