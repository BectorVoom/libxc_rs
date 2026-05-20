//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2835/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2835<F: Float>(t10858: F, t23257: F, t221: F, t23279: F, t10703: F, t2674: F, t2661: F, t2662: F, t6035: F, t61579: F, t10698: F, t1544: F, t18392: F, t2477: F, t40625: F, t40638: F, t40639: F, t40654: F, t40691: F, t40711: F, t4343: F, t50446: F, t50703: F, t50707: F, t5962: F, t5966: F, t61860: F, t61864: F, t61877: F, t775: F, t828: F, t851: F) -> F {
    let t76596 = t10858 * t23257;
    let t76613 = t221 * t23279;
    let t76615 = t2674 * t10703 * t76613;
    let t76619 = t2661 * t2662 * t61579 * t6035;
    let t76633 = -F::cast_from(0.60023625365297631763e-2_f64) * t76596 - F::cast_from(0.77173232612525526549e-1_f64) * t851 * t10698 * t828 * t5966 * t4343 + F::cast_from(0.12862205435420921092e-1_f64) * t851 * t2477 * t828 * t4343 * t5962 + F::cast_from(0.12862205435420921092e-1_f64) * t851 * t2477 * t828 * t1544 * t18392 + F::cast_from(0.7623000421392799234e-3_f64) * t76615 - F::cast_from(0.85748036236139473942e-4_f64) * t76619 + F::cast_from(0.45178982497454656792e-6_f64) * t40625 - t40638 + F::cast_from(0.28900264064772933811e-2_f64) * t40639 + t40654 + F::cast_from(0.12862205435420921092e-3_f64) * t61860 - F::cast_from(0.12862205435420921092e-3_f64) * t61864 + F::cast_from(0.30492001685571196935e-4_f64) * t61877 - F::new(3.0) / F::new(4.0) * t50446 * t221 * t23279 * t775 + F::cast_from(0.97586602194502058671e-3_f64) * t50703 - t50707 + F::cast_from(0.11294745624363664198e-6_f64) * t40691 - F::cast_from(0.51384669507166276316e-2_f64) * t40711;
    t76633
}
