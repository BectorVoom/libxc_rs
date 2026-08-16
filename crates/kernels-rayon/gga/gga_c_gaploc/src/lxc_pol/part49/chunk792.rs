//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 792/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk792(t13847: f64, t2685: f64, t2684: f64, t969: f64, t825: f64, t13010: f64, t13015: f64, t13018: f64, t13021: f64, t13026: f64, t13028: f64, t13029: f64, t13031: f64, t13036: f64, t13040: f64, t13044: f64, t13047: f64) -> (f64, f64, f64) {
    let t13848 = t2685 * t13847;
    let t13849 = t2684 * t13848;
    let t13851 = t969 * t13847;
    let t13852 = t825 * t13851;
    let t13854 = -0.69017266717057349418e1_f64 * t13010 - t13015 - t13018 + 0.71500979903700853338e0_f64 * t13021 + t13026 + t13028 + 0.35750489951850426669e0_f64 * t13029 - 0.10725146985555128001e1_f64 * t13031 + t13036 - t13040 + t13044 - t13047 + 0.19171462976960374838e0_f64 * t13849 - 0.19171462976960374838e0_f64 * t13852;
    (t13848, t13851, t13854)
}
