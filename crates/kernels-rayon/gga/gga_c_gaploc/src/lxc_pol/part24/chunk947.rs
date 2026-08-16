//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 947/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk947(t10286: f64, t2497: f64, t2798: f64, t1016: f64, t6553: f64, t2801: f64, t6556: f64, t2355: f64, t2902: f64, t3366: f64, t4342: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10287 = 2.0_f64 * t10286;
    let t10288 = t2798 * t2497;
    let t10289 = t6553 * t1016;
    let t10290 = t6556 * t2801;
    let t10291 = 2.0_f64 * t10290;
    let t10292 = t2355 * t2902;
    let t10293 = t4342 * t3366;
    let t10294 = 2.0_f64 * t10293;
    let t10295 = t3366 * t605;
    (t10287, t10288, t10289, t10290, t10291, t10292, t10293, t10294, t10295)
}
