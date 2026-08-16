//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1230/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1230(t32435: f64, t7290: f64, t1841: f64, t7289: f64, t10755: f64, t5288: f64, t10683: f64, t7129: f64, t1897: f64, t2717: f64, t8942: f64, t10643: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32436 = t7290 * t32435;
    let t32439 = 0.34180116578409885704e-2_f64 * t1841 * t7289 * t32436;
    let t32441 = 0.15381052460284448567e-1_f64 * t5288 * t10755;
    let t32443 = 0.15381052460284448567e-1_f64 * t7129 * t10683;
    let t32446 = 0.15381052460284448567e-1_f64 * t1897 * t2717 * t8942;
    let t32448 = 0.10766736722199113997e0_f64 * t7129 * t10643;
    (t32436, t32439, t32441, t32443, t32446, t32448)
}
