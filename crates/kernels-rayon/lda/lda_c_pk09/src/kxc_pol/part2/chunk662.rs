//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 662/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk662(t1435: f64, t1594: f64, t1597: f64, t1610: f64, t747: f64, t1609: f64, t303: f64, t337: f64, t280: f64, t1303: f64, t1625: f64, t1349: f64, t5164: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6018 = t1594 * t1435;
    let t6020 = t1597 * t1435;
    let t6022 = t747 * t1610;
    let t6023 = t1609 * t6022;
    let t6025 = t303 * t337;
    let t6026 = t6025 * t280;
    let t6027 = t1303 * t6026;
    let t6028 = t6027 * t1625;
    let t6030 = t1349 * t5164;
    (t6018, t6020, t6022, t6023, t6026, t6028, t6030)
}
