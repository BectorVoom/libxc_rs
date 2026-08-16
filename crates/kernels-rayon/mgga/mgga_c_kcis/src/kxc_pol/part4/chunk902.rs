//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 902/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk902(t449: f64, t6260: f64, t446: f64, t4529: f64, t113: f64, t774: f64, t2150: f64, t62: f64, t822: f64, t251: f64, t4863: f64, t2532: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6261 = t449 * t6260;
    let t6262 = t446 * t6261;
    let t6292 = 2.0_f64 * t4529;
    let t7617 = t113 * t774;
    let t7624 = t2150 * t774;
    let t7627 = t62 * t822;
    let t8291 = t251 * t4863;
    let t8521 = 3.0_f64 * t2532;
    (t6262, t6292, t7617, t7624, t7627, t8291, t8521)
}
