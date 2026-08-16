//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1079/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1079(t1020: f64, t18473: f64, t2811: f64, t6544: f64, t1008: f64, t6301: f64, t9985: f64, t4781: f64, t4977: f64, t2861: f64, t6563: f64, t4999: f64, t5013: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18474 = t1020 * t18473;
    let t18476 = t6544 * t2811;
    let t18477 = t18476 * t1008;
    let t18482 = t6301 * t9985;
    let t18483 = t18482 * t1008;
    let t18486 = t4781 * t4977;
    let t18495 = t2861 * t6563;
    let t18497 = t4999 * t5013;
    (t18474, t18477, t18483, t18486, t18495, t18497)
}
