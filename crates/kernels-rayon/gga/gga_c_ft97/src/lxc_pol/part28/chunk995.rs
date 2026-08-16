//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 995/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk995(t1882: f64, t33157: f64, t33031: f64, t8392: f64, t33028: f64, t33009: f64, t33062: f64, t33138: f64, t33082: f64, t33017: f64, t7402: f64, t8232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t140161 = t1882 * t33157;
    let t140169 = t8392 * t33031;
    let t140237 = t1882 * t33028;
    let t140239 = t1882 * t33009;
    let t140241 = t1882 * t33062;
    let t140253 = t1882 * t33138;
    let t140263 = t1882 * t33082;
    let t140268 = t1882 * t33017;
    let t140274 = 8.0_f64 / 27.0_f64 * t8232 * t7402;
    (t140161, t140169, t140237, t140239, t140241, t140253, t140263, t140268, t140274)
}
