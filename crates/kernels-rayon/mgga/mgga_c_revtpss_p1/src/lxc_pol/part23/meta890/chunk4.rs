//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2835/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2835(t10858: f64, t23257: f64, t221: f64, t23279: f64, t10703: f64, t2674: f64, t2661: f64, t2662: f64, t6035: f64, t61579: f64, t10698: f64, t1544: f64, t18392: f64, t2477: f64, t40625: f64, t40638: f64, t40639: f64, t40654: f64, t40691: f64, t40711: f64, t4343: f64, t50446: f64, t50703: f64, t50707: f64, t5962: f64, t5966: f64, t61860: f64, t61864: f64, t61877: f64, t775: f64, t828: f64, t851: f64) -> f64 {
    let t76596 = t10858 * t23257;
    let t76613 = t221 * t23279;
    let t76615 = t2674 * t10703 * t76613;
    let t76619 = t2661 * t2662 * t61579 * t6035;
    let t76633 = -0.60023625365297631763e-2_f64 * t76596 - 0.77173232612525526549e-1_f64 * t851 * t10698 * t828 * t5966 * t4343 + 0.12862205435420921092e-1_f64 * t851 * t2477 * t828 * t4343 * t5962 + 0.12862205435420921092e-1_f64 * t851 * t2477 * t828 * t1544 * t18392 + 0.7623000421392799234e-3_f64 * t76615 - 0.85748036236139473942e-4_f64 * t76619 + 0.45178982497454656792e-6_f64 * t40625 - t40638 + 0.28900264064772933811e-2_f64 * t40639 + t40654 + 0.12862205435420921092e-3_f64 * t61860 - 0.12862205435420921092e-3_f64 * t61864 + 0.30492001685571196935e-4_f64 * t61877 - 3.0_f64 / 4.0_f64 * t50446 * t221 * t23279 * t775 + 0.97586602194502058671e-3_f64 * t50703 - t50707 + 0.11294745624363664198e-6_f64 * t40691 - 0.51384669507166276316e-2_f64 * t40711;
    t76633
}
