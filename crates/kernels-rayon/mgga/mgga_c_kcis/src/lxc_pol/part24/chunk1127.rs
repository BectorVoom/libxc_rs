//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1127/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1127(t1646: f64, t4972: f64, t1003: f64, t6272: f64, t167: f64, t1704: f64, t6276: f64, t6544: f64, t9985: f64, t2835: f64, t6432: f64, t1141: f64, t19824: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t71184 = t1646 * t4972;
    let t71203 = t6272 * t1003;
    let t71215 = t167 * t1704;
    let t71387 = t6276 * t1003;
    let t71722 = t6544 * t9985;
    let t71731 = t6432 * t2835;
    let t71840 = t19824 * t1141;
    (t71184, t71203, t71215, t71387, t71722, t71731, t71840)
}
