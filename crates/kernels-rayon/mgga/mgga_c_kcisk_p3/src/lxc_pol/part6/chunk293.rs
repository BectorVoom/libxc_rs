//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 293/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk293(t538: f64, t398: f64, t544: f64, t1334: f64, t554: f64, t551: f64, t298: f64, t430: f64, t569: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1587 = t538 * t538;
    let t1588 = 1.0_f64 / t1587;
    let t1589 = t398 * t1588;
    let t1597 = 1.0_f64 / t544;
    let t1601 = 0.11607361111111111111e-2_f64 * t1334;
    let t1609 = t554 * t554;
    let t1610 = 1.0_f64 / t1609;
    let t1611 = t551 * t1610;
    let t1634 = t298 * t430 * t569;
    (t1587, t1588, t1589, t1597, t1601, t1609, t1610, t1611, t1634)
}
