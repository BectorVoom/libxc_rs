//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 826/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk826(t35323: f64, t7515: f64, t7511: f64, t7512: f64, t2506: f64, t35309: f64, t193: f64, t6109: f64, t1091: f64, t33319: f64, t9770: f64, t6118: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35324 = t7515 * t35323;
    let t35326 = t7511 * t7512 * t35324;
    let t35328 = t2506 * t35309;
    let t35330 = t6109 * t193 * t35328;
    let t35333 = t9770 * t33319 * t1091;
    let t35334 = t6118 * t35333;
    (t35324, t35326, t35328, t35330, t35333, t35334)
}
