//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 679/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk679(t23443: f64, t3425: f64, t569: f64, t5975: f64, t925: f64, t3578: f64, t574: f64, t5869: f64, t2142: f64, t6639: f64, t1053: f64, t5842: f64) -> (f64, f64, f64, f64, f64) {
    let t26868 = t23443 * t3425;
    let t26872 = t569 * t5975 * t925;
    let t26876 = t574 * t3578 * t5869;
    let t26880 = t574 * t2142 * t6639;
    let t26883 = t5842 * t1053;
    (t26868, t26872, t26876, t26880, t26883)
}
