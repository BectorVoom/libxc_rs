//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1252/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1252(t12747: f64, t1755: f64, t1182: f64, t1410: f64, t5956: f64, t997: f64, t5816: f64, t1150: f64, t1165: f64, t1180: f64, t1181: f64, t12945: f64, t12946: f64, t13802: f64, t1532: f64, t17694: f64, t17701: f64, t17703: f64, t17708: f64, t20092: f64, t336: f64, t337: f64, t4463: f64, t4680: f64, t5852: f64, t5927: f64) -> f64 {
    let t23039 = t12747 * t1755;
    let t23045 = t1182 * t1410;
    let t23055 = t997 * t5956;
    let t23060 = t997 * t5816;
    let t23062 = 0.85748036236139473945e-2_f64 * t12945 * t1165 * t5852 * t12946 - 0.22675591804667994221e-1_f64 * t23039 + 0.34299214494455789578e-1_f64 * t4463 * t4680 * t5927 + 0.17149607247227894789e-2_f64 * t13802 - 0.17149607247227894789e-2_f64 * t1180 * t1181 * t1532 * t23045 - 0.16006300097412701803e-1_f64 * t17694 + t1150 * t336 * t337 * t20092 / 8.0_f64 + 0.32012600194825403606e-1_f64 * t23055 + 0.13605355082800796533e0_f64 * t17701 + 0.17149607247227894789e-2_f64 * t17703 - 0.32012600194825403606e-1_f64 * t17708 + 0.16006300097412701803e-1_f64 * t23060;
    t23062
}
