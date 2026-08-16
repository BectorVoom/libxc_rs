//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 379/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk379(t2343: f64, t3354: f64, t2268: f64, t1016: f64, t921: f64, t2877: f64, t895: f64, t2898: f64, t901: f64, t1645: f64, t888: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3355 = t2343 * t3354;
    let t3357 = 0.56910013271352299198e-1_f64 * t2268 * t3355;
    let t3366 = t1016 * t921;
    let t3370 = 0.35750489951850426669e0_f64 * t895 * t2877;
    let t3375 = t2898 * t901;
    let t3376 = 0.14896037479937677779e-1_f64 * t3375;
    let t3377 = t1645 * t888;
    (t3355, t3357, t3366, t3370, t3375, t3376, t3377)
}
