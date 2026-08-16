//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1009/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1009(t16899: f64, t4465: f64, t1462: f64, t3670: f64, t4999: f64, t952: f64, t5108: f64, t997: f64, t1008: f64, t5255: f64, t10098: f64, t1162: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17002 = t16899 * t4465;
    let t17016 = t3670 * t1462;
    let t17018 = t952 * t4999;
    let t17020 = t997 * t5108;
    let t17029 = t1008 * t5255;
    let t17039 = t10098 * t1162;
    (t17002, t17016, t17018, t17020, t17029, t17039)
}
