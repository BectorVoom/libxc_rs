//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1268/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1268(t1890: f64, t1966: f64, t32435: f64, t590: f64, t326: f64, t32889: f64, t7394: f64, t28412: f64, t8970: f64, t913: f64, t1022: f64, t15499: f64) -> (f64, f64, f64, f64) {
    let t33526 = 0.51123901271894332902e1_f64 * t1966 * t1890 * t32435 * t590;
    let t33529 = 0.92023022289409799224e1_f64 * t7394 * t326 * t32889;
    let t33531 = t28412 * t913 * t8970;
    let t33532 = 0.59584149919750711116e-1_f64 * t33531;
    let t33533 = t15499 * t1022;
    (t33526, t33529, t33532, t33533)
}
