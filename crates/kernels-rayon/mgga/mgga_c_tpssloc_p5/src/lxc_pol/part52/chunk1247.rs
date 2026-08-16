//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1247/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1247(t226: f64, t235: f64, t2690: f64, t8344: f64, t23139: f64, t8339: f64, t23171: f64, t23228: f64, t8335: f64, t30623: f64, t81651: f64, t82074: f64) -> (f64, f64, f64, f64) {
    let t112850 = t226 * t235 * t2690 * t8344;
    let t112851 = 119.0_f64 / 6912.0_f64 * t112850;
    let t112855 = t23139 * t8339;
    let t112856 = 0.45217486825437237757e-1_f64 * t112855;
    let t112863 = 0.16449340668482264365e-1_f64 * t23171 * t23228 * t8335;
    let t112867 = t81651 * t82074 * t30623;
    (t112851, t112856, t112863, t112867)
}
