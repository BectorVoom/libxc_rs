//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 781/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk781<F: Float>(t2597: F, t5397: F, t2386: F, t4787: F, t4761: F, t5372: F, t12261: F, t2643: F, t782: F, t2618: F, t5444: F, t2656: F, t5531: F) -> (F, F, F, F, F, F, F) {
    let t18546 = t2597 * t5397;
    let t18558 = t2386 * t4787;
    let t18640 = t2386 * t4761;
    let t18643 = t2597 * t5372;
    let t18700 = t12261 * t2643;
    let t18701 = t782 * t18700;
    let t18779 = t2618 * t5444;
    let t18925 = t2656 * t5531;
    (t18546, t18558, t18640, t18643, t18701, t18779, t18925)
}
