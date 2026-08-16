//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1150/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1150(t18970: f64, t3163: f64, t10530: f64, t584: f64, t6574: f64, t123: f64, t18313: f64, t20369: f64, t883: f64, t6907: f64, t888: f64, t9263: f64) -> (f64, f64, f64, f64) {
    let t31068 = 0.29792074959875355558e-1_f64 * t18970 * t3163;
    let t31119 = t584 * t10530 * t6574;
    let t31120 = t18313 * t123;
    let t31124 = 0.46011511144704899612e1_f64 * t31119 * t31120 * t883 * t20369;
    let t31126 = t9263 * t888 * t6907;
    (t31068, t31119, t31124, t31126)
}
