//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 848/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk848(t107: f64, t2760: f64, t1415: f64, t1359: f64, t2875: f64, t544: f64, t4820: f64, t7906: f64, t1339: f64, t2754: f64, t590: f64, t2792: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8261 = t2760 * t107;
    let t8262 = t1415 * t8261;
    let t8265 = t1359 * t2875;
    let t8266 = t544 * t8265;
    let t8269 = t4820 * t7906;
    let t8272 = t1339 * t2754;
    let t8273 = t8272 * t590;
    let t8278 = t2792 * t590;
    (t8262, t8265, t8266, t8269, t8272, t8273, t8278)
}
