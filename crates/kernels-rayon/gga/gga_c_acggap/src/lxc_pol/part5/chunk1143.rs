//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1143/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1143(t1165: f64, t1173: f64, t12720: f64, t12724: f64, t12728: f64, t12734: f64, t12736: f64, t12739: f64, t12744: f64, t12748: f64, t12750: f64, t15550: f64, t15560: f64, t1748: f64, t175: f64, t20175: f64, t397: f64, t398: f64) -> f64 {
    let t20517 = 0.60023625365297631762e-2_f64 * t12720 - 0.42874018118069736972e-3_f64 * t397 * t398 * t175 * t20175 + 0.20007875121765877254e-2_f64 * t12724 + 0.85748036236139473944e-3_f64 * t12728 - 0.12004725073059526352e-1_f64 * t15550 + 0.25724410870841842184e-2_f64 * t12734 + 0.42874018118069736972e-3_f64 * t12736 + 0.17149607247227894789e-2_f64 * t1173 * t1165 * t15560 * t1748 + 0.40015750243531754508e-2_f64 * t12739 + 0.45351183609335988442e-1_f64 * t12744 - 0.22675591804667994222e-1_f64 * t12748 + 0.22675591804667994222e-1_f64 * t12750;
    t20517
}
