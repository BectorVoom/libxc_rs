//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 719/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk719(t13048: f64, t13075: f64, t13123: f64, t13164: f64, t13101: f64, t738: f64, t13096: f64, t169: f64, t299: f64, t706: f64, t2558: f64, t3464: f64) -> (f64, f64, f64, f64, f64) {
    let t13166 = t13048 + t13075 + t13123 + t13164;
    let t13168 = t738 * t13101;
    let t13172 = t13096 * t169 * t299;
    let t13173 = t706 * t13172;
    let t13176 = t3464 * t2558;
    (t13166, t13168, t13172, t13173, t13176)
}
