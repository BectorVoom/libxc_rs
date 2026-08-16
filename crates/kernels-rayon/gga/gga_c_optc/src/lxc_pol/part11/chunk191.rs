//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 191/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk191(t50: f64, t277: f64, t391: f64, t419: f64, t421: f64, t475: f64, t490: f64, t498: f64, t95: f64, t7: f64, t8: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t501 = -t391 + t419 + t421 + 0.25844881434903430496e-2_f64 * t95 * t277 * t475 + t490 * t498 / 2.0_f64;
    let t502 = piecewise3(t51, zeta_threshold, t50);
    let t507 = t8 * t7;
    let t508 = 1.0_f64 / t507;
    (t501, t502, t508)
}
