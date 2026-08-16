//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 797/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk797(t4990: f64, t7947: f64, t2641: f64, t4941: f64, t5025: f64, t8152: f64, t2721: f64, t1382: f64, t8384: f64, t2434: f64, t2367: f64, t5021: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14488 = t7947 * t4990;
    let t14525 = t2641 * t4941;
    let t14538 = t8152 * t5025;
    let t14539 = t2721 * t14538;
    let t14578 = t8384 * t1382;
    let t14585 = t2434 * t1382;
    let t14599 = t2367 * t5021;
    (t14488, t14525, t14539, t14578, t14585, t14599)
}
