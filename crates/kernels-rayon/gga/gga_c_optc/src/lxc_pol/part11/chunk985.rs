//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 985/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk985(t18023: f64, t3092: f64, t914: f64, t1150: f64, t1162: f64, t12578: f64, t12581: f64, t17994: f64, t18006: f64, t18009: f64, t18013: f64, t18016: f64, t18020: f64, t3103: f64, t3234: f64, t4435: f64, t4492: f64, t4501: f64, t5389: f64, t5394: f64, t5399: f64, t5404: f64) -> (f64, f64, f64) {
    let t18024 = t3092 * t18023;
    let t18025 = t914 * t18024;
    let t18028 = 0.11360101276506094136e1_f64 * t1150 * t17994 - 0.90880810212048753088e1_f64 * t4501 * t5404 + 0.35163949364965747848e4_f64 * t12581 * t5399 - 0.70327898729931495696e4_f64 * t12578 * t5394 - 0.9356877183176434872e2_f64 * t4492 * t5389 + 0.4645868436449114021e2_f64 * t4435 * t18006 + 0.1169609647897054359e2_f64 * t3234 * t18009 + 0.1949349413161757265e2_f64 * t3234 * t18013 - 0.15486228121497046737e2_f64 * t3103 * t18016 + 0.1169609647897054359e2_f64 * t3234 * t18020 - 0.17386322979577515709e0_f64 * t1162 * t18025;
    (t18024, t18025, t18028)
}
