//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1355/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1355(t35228: f64, t10434: f64, t1391: f64, t2487: f64, t2355: f64, t8435: f64, t27229: f64, t7826: f64, t10289: f64, t10299: f64, t10293: f64, t10302: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35229 = 0.51123901271894332902e0_f64 * t35228;
    let t35231 = t2487 * t1391 * t10434;
    let t35232 = 0.2698205900461089792e0_f64 * t35231;
    let t35240 = t2355 * t8435;
    let t35242 = 6.0_f64 * t27229 * t7826;
    let t35252 = 2.0_f64 * t10289;
    let t35253 = 4.0_f64 * t10299;
    let t35254 = 4.0_f64 * t10293;
    let t35255 = 4.0_f64 * t10302;
    (t35229, t35232, t35240, t35242, t35252, t35253, t35254, t35255)
}
