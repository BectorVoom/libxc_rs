//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1012/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1012(t1008: f64, t5003: f64, t12747: f64, t1549: f64, t1554: f64, t1558: f64, t3431: f64, t4291: f64, t3409: f64, t4295: f64, t1535: f64, t3371: f64, t4180: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17092 = t1008 * t5003;
    let t17105 = t12747 * t1549;
    let t17107 = t12747 * t1554;
    let t17109 = t12747 * t1558;
    let t17111 = t3431 * t4291;
    let t17113 = t3409 * t4295;
    let t17116 = t4180 * t3371 * t1535;
    (t17092, t17105, t17107, t17109, t17111, t17113, t17116)
}
