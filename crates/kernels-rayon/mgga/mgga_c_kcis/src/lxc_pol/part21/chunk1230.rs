//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1230/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1230(t27006: f64, t27014: f64, t10995: f64, t7787: f64, t26954: f64, t27076: f64, t26996: f64, t993: f64, t1095: f64, t982: f64, t11081: f64, t26960: f64, t26962: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92607 = t27014 * t27006;
    let t92613 = t7787 * t10995;
    let t92657 = t27076 * t26954;
    let t92693 = t993 * t26996;
    let t92701 = t1095 * t982;
    let t92718 = t26960 * t11081 * t26962;
    (t92607, t92613, t92657, t92693, t92701, t92718)
}
