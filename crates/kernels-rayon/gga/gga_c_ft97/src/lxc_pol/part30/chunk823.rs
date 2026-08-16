//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 823/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk823(t1403: f64, t35251: f64, t35255: f64, t35259: f64, t35263: f64, t35267: f64, t35270: f64, t35276: f64, t35282: f64, t35287: f64, t35297: f64, t35302: f64, t35304: f64, t6002: f64, t6745: f64, t6754: f64, t6840: f64, t6844: f64, t7437: f64, t7443: f64, t7487: f64, t7491: f64) -> f64 {
    let t35306 = -t6002 * t35251 / 18.0_f64 + t6002 * t35255 / 9.0_f64 - t6002 * t35259 / 9.0_f64 + t1403 * t35263 / 3.0_f64 + t1403 * t35267 - 2.0_f64 / 3.0_f64 * t1403 * t35270 - t7437 * t6754 / 3.0_f64 + t1403 * t35276 / 3.0_f64 + t6745 * t7491 / 3.0_f64 + t1403 * t35282 / 6.0_f64 - 2.0_f64 / 3.0_f64 * t1403 * t35287 - t6745 * t7443 / 3.0_f64 + t7437 * t6844 / 6.0_f64 + t7437 * t6840 / 6.0_f64 - t1403 * t35297 / 3.0_f64 + t6745 * t7487 / 6.0_f64 - 4.0_f64 * t35302 - 4.0_f64 * t35304;
    t35306
}
