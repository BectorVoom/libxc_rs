//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 569/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk569(t1470: f64, t3382: f64, t1562: f64, t3379: f64, t1466: f64, t1545: f64, t3431: f64, t1524: f64, t322: f64, t1095: f64, t398: f64, t384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4689 = 0.85748036236139473944e-3_f64 * t3382 * t1470;
    let t4699 = 0.17149607247227894789e-2_f64 * t3379 * t1562;
    let t4705 = 0.85748036236139473944e-3_f64 * t3382 * t1466;
    let t4716 = t3431 * t1545;
    let t4718 = t1524 * t322;
    let t4720 = t398 * t1095 * t4718;
    let t4722 = 0.85748036236139473944e-3_f64 * t384 * t4720;
    (t4689, t4699, t4705, t4716, t4718, t4720, t4722)
}
