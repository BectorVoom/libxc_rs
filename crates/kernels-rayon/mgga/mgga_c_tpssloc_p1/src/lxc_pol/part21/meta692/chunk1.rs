//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2508/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2508(t13080: f64, t9638: f64, t226: f64, t40931: f64, t68: f64, t13377: f64, t814: f64, t13396: f64, t808: f64, t13068: f64, t225: f64, t13030: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47353 = t9638 * t13080;
    let t47386 = t226 * t68 * t40931;
    let t47395 = t814 * t13377;
    let t47419 = t808 * t13396;
    let t47568 = t13068 * t225;
    let t47585 = t13030 * t225;
    (t47353, t47386, t47395, t47419, t47568, t47585)
}
