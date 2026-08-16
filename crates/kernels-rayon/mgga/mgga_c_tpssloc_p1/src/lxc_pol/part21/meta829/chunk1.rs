//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2923/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2923(t4696: f64, t13732: f64, t4483: f64, t4471: f64, t950: f64, t14369: f64, t49513: f64, t4475: f64, t49532: f64, t4496: f64, t48883: f64, t959: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60880 = t4696 * t4696;
    let t60886 = 0.23392894490538584828e1_f64 * t4483 * t13732;
    let t60887 = t4471 * t950;
    let t60890 = 0.41016075432865626631e4_f64 * t49513 * t14369 * t60887;
    let t60893 = 0.4155806185363551302e3_f64 * t49532 * t4475 * t60887;
    let t60899 = 0.34631718211362927518e2_f64 * t959 * t4496 * t48883;
    (t60880, t60886, t60887, t60890, t60893, t60899)
}
