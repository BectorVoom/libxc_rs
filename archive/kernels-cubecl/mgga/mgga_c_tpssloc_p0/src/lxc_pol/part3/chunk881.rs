//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 881/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk881<F: Float>(t812: F, t9670: F, t831: F, t2617: F, t2638: F, t2639: F, t2681: F, t116: F, t126: F, t136: F, t16: F, t2386: F, t625: F) -> (F, F, F, F, F, F, F) {
    let t9671 = t812 * t9670;
    let t9672 = t9671 * t831;
    let t9674 = t2617 * t2638;
    let t9675 = t9674 * t831;
    let t9679 = t2639 * t2681;
    let t9688 = F::cast_from(1.0_f64) / t126 / t136 * t116 / F::cast_from(4.0_f64);
    let t9689 = t9688 * t16;
    let t9691 = t2386 * t625;
    (t9671, t9672, t9674, t9675, t9679, t9689, t9691)
}
