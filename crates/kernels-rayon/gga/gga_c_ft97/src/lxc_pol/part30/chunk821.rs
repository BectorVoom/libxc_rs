//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 821/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk821(t1091: f64, t33494: f64, t9770: f64, t2354: f64, t33502: f64, t1425: f64, t6945: f64, t193: f64, t33243: f64, t6752: f64, t27991: f64, t6008: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35255 = t9770 * t33494 * t1091;
    let t35259 = t2354 * t33502 * t1091;
    let t35262 = t1425 * t6945;
    let t35263 = t193 * t35262;
    let t35266 = t33243 * t6752;
    let t35267 = t193 * t35266;
    let t35269 = t6008 * t27991;
    (t35255, t35259, t35262, t35263, t35266, t35267, t35269)
}
