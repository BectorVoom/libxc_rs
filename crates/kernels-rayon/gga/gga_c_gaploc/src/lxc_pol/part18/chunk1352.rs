//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1352/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1352(t14630: f64, t2859: f64, t888: f64, t2877: f64, t6866: f64, t6773: f64, t2437: f64, t8072: f64, t10144: f64, t4614: f64, t597: f64, t1: f64, t10215: f64, t106: f64, t192: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34119 = 0.23833659967900284447e0_f64 * t2859 * t14630 * t888;
    let t34121 = 0.35750489951850426669e0_f64 * t6866 * t2877;
    let t34123 = 0.71500979903700853338e0_f64 * t6773 * t2877;
    let t34125 = 0.71500979903700853338e0_f64 * t2437 * t8072;
    let t34128 = 0.30674340763136599742e2_f64 * t597 * t4614 * t10144;
    let t34131 = t10215 * t1 * t106 * t192;
    (t34119, t34121, t34123, t34125, t34128, t34131)
}
