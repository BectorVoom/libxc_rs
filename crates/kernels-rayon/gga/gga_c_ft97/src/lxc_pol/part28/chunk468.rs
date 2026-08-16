//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 468/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk468(t103: f64, t7264: f64, t82: f64, t1332: f64, t5710: f64, t83: f64) -> (f64, f64, f64, f64) {
    let t7266 = t82 * t7264 * t103;
    let t7270 = t5710 * t1332;
    let t7271 = t83 * t7270;
    let t7274 = t1332 * t1332;
    (t7266, t7270, t7271, t7274)
}
