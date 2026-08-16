//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 562/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk562(t1466: f64, t3409: f64, t1106: f64, t1181: f64, t540: f64, t3391: f64, t1162: f64, t4198: f64, t1541: f64, t3375: f64, t1545: f64, t3379: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4423 = 0.40015750243531754508e-2_f64 * t3409 * t1466;
    let t4425 = t1181 * t540 * t1106;
    let t4427 = 0.17149607247227894789e-2_f64 * t3391 * t4425;
    let t4450 = t4198 * t1162;
    let t4459 = t3375 * t1541;
    let t4462 = 0.17149607247227894789e-2_f64 * t3379 * t1545;
    (t4423, t4425, t4427, t4450, t4459, t4462)
}
