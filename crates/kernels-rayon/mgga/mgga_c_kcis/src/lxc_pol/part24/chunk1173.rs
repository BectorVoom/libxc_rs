//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1173/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1173(t1095: f64, t982: f64, t7720: f64, t9562: f64, t27076: f64, t3489: f64, t34690: f64, t421: f64, t46978: f64, t7774: f64, t7772: f64, t1250: f64, t251: f64, t35547: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92701 = t1095 * t982;
    let t92730 = t9562 * t7720;
    let t92732 = t27076 * t3489;
    let t92735 = t421 * t34690;
    let t92748 = t46978 * t7774;
    let t92749 = t7772 * t92748;
    let t92761 = t35547 * t251 * t1250;
    (t92701, t92730, t92732, t92735, t92748, t92749, t92761)
}
