//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 827/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk827(t1022: f64, t6613: f64, t1096: f64, t1092: f64, t1646: f64, t1767: f64, t3203: f64, t3202: f64, t3200: f64, t1773: f64, t3211: f64, t3210: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6614 = t1022 * t6613;
    let t6615 = t1096 * t6614;
    let t6616 = t1092 * t6615;
    let t6619 = t1646 * t1767;
    let t6620 = t3203 * t6619;
    let t6621 = t3202 * t6620;
    let t6622 = t3200 * t6621;
    let t6624 = t1646 * t1773;
    let t6625 = t3211 * t6624;
    let t6626 = t3210 * t6625;
    (t6614, t6615, t6616, t6620, t6621, t6622, t6625, t6626)
}
