//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1177/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1177(t4297: f64, t53260: f64, t16094: f64, t4230: f64, t15008: f64, t4539: f64, t11782: f64, t17515: f64, t18187: f64, t4281: f64, t9142: f64, t15083: f64, t15107: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53261 = t4297 * t53260;
    let t53279 = t4230 * t16094;
    let t53281 = t15008 * t4539;
    let t53289 = t11782 * t17515;
    let t53290 = t4297 * t53289;
    let t53293 = t4281 * t9142 * t18187;
    let t53299 = t15083 * t15107;
    (t53261, t53279, t53281, t53290, t53293, t53299)
}
