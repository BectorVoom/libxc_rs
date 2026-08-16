//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 617/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk617(t1022: f64, t6613: f64, t1096: f64, t1092: f64, t1646: f64, t1767: f64, t3203: f64) -> (f64, f64, f64, f64) {
    let t6614 = t1022 * t6613;
    let t6615 = t1096 * t6614;
    let t6616 = t1092 * t6615;
    let t6619 = t1646 * t1767;
    let t6620 = t3203 * t6619;
    (t6614, t6615, t6616, t6620)
}
