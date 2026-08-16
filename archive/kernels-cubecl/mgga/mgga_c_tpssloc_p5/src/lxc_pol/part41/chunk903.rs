//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 903/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk903<F: Float>(t2617: F, t2638: F, t116: F, t126: F, t136: F, t16: F, t2386: F, t625: F, t2385: F, t686: F, t781: F, t685: F) -> (F, F, F, F, F, F) {
    let t9674 = t2617 * t2638;
    let t9688 = F::cast_from(1.0_f64) / t126 / t136 * t116 / F::cast_from(4.0_f64);
    let t9689 = t9688 * t16;
    let t9691 = t2386 * t625;
    let t9692 = t2385 * t9691;
    let t9694 = t686 * t781;
    let t9695 = t685 * t9694;
    (t9674, t9689, t9691, t9692, t9694, t9695)
}
