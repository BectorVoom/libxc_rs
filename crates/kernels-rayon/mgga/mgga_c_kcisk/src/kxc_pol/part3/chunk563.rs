//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 563/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk563(t1665: f64, t4699: f64, t1643: f64, t583: f64, t573: f64, t1663: f64, t1664: f64, t4636: f64, t571: f64, t4624: f64, t1653: f64, t4652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4701 = 2.0_f64 * t4699 * t1665;
    let t4702 = t1643 * t583;
    let t4703 = 1.0_f64 / t4702;
    let t4704 = t573 * t4703;
    let t4705 = t1663 * t1663;
    let t4706 = t4705 * t1664;
    let t4708 = 2.0_f64 * t4704 * t4706;
    let t4711 = 0.39862222222222222223e0_f64 * t4636;
    let t4716 = 1.0_f64/f64::sqrt(t571);
    let t4717 = t4716 * t4624;
    let t4719 = t1653 * t4652;
    (t4701, t4703, t4704, t4705, t4706, t4708, t4711, t4716, t4717, t4719)
}
