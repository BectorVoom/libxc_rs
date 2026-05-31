//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1252/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1252<F: Float>(t12747: F, t1755: F, t1182: F, t1410: F, t5956: F, t997: F, t5816: F, t1150: F, t1165: F, t1180: F, t1181: F, t12945: F, t12946: F, t13802: F, t1532: F, t17694: F, t17701: F, t17703: F, t17708: F, t20092: F, t336: F, t337: F, t4463: F, t4680: F, t5852: F, t5927: F) -> F {
    let t23039 = t12747 * t1755;
    let t23045 = t1182 * t1410;
    let t23055 = t997 * t5956;
    let t23060 = t997 * t5816;
    let t23062 = F::cast_from(0.85748036236139473945e-2_f64) * t12945 * t1165 * t5852 * t12946 - F::cast_from(0.22675591804667994221e-1_f64) * t23039 + F::cast_from(0.34299214494455789578e-1_f64) * t4463 * t4680 * t5927 + F::cast_from(0.17149607247227894789e-2_f64) * t13802 - F::cast_from(0.17149607247227894789e-2_f64) * t1180 * t1181 * t1532 * t23045 - F::cast_from(0.16006300097412701803e-1_f64) * t17694 + t1150 * t336 * t337 * t20092 / F::cast_from(8.0_f64) + F::cast_from(0.32012600194825403606e-1_f64) * t23055 + F::cast_from(0.13605355082800796533e0_f64) * t17701 + F::cast_from(0.17149607247227894789e-2_f64) * t17703 - F::cast_from(0.32012600194825403606e-1_f64) * t17708 + F::cast_from(0.16006300097412701803e-1_f64) * t23060;
    t23062
}
