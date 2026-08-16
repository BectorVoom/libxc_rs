//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1070/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1070(t4167: f64, t4180: f64, t377: f64, t4251: f64, t3073: f64, t5315: f64, t945: f64, t1160: f64, t4146: f64, t4162: f64, t4166: f64, t15758: f64, t1629: f64, t3088: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19000 = t4180 * t4167;
    let t19005 = t377 * t4251;
    let t19015 = t3073 * t5315 * t945;
    let t19023 = t1160 * t4146 * t4162;
    let t19026 = t1160 * t4166 * t4162;
    let t19029 = t3088 * t1629 * t15758;
    (t19000, t19005, t19015, t19023, t19026, t19029)
}
