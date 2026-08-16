//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 971/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk971(t28276: f64, t30663: f64, t6552: f64, t1484: f64, t1527: f64, t22986: f64, t23270: f64, t30633: f64, t118885: f64, t118893: f64, t1880: f64, t28294: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126412 = 0.3289868133696452873e-1_f64 * t6552 * t30663 * t28276;
    let t126413 = t1484 * t1527;
    let t126417 = 0.13159472534785811492e0_f64 * t22986 * t23270 * t30633 * t126413;
    let t126418 = 0.16449340668482264365e-1_f64 * t118885;
    let t126419 = 0.76763589786250567036e-1_f64 * t118893;
    let t126422 = 0.3289868133696452873e-1_f64 * t1880 * t30663 * t28294;
    (t126412, t126413, t126417, t126418, t126419, t126422)
}
