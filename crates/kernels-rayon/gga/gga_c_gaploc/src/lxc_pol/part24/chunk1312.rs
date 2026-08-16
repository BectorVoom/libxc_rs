//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1312/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1312(t326: f64, t32889: f64, t7394: f64, t28412: f64, t8970: f64, t913: f64, t1022: f64, t15499: f64, t28640: f64, t7419: f64, t3005: f64, t7383: f64, t9800: f64) -> (f64, f64, f64, f64) {
    let t33529 = 0.92023022289409799224e1_f64 * t7394 * t326 * t32889;
    let t33531 = t28412 * t913 * t8970;
    let t33532 = 0.59584149919750711116e-1_f64 * t33531;
    let t33533 = t15499 * t1022;
    let t33535 = t28640 * t33533 * t7419;
    let t33536 = 0.23005755572352449806e1_f64 * t33535;
    let t33538 = t9800 * t3005 * t7383;
    (t33529, t33532, t33536, t33538)
}
