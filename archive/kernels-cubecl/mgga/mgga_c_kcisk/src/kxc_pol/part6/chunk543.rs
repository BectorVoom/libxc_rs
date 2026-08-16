//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 543/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk543<F: Float>(t2637: F, t4998: F, t2013: F, t2643: F, t4419: F, t782: F, t2642: F, t5507: F, t1993: F, t2618: F, t2041: F, t2656: F) -> (F, F, F, F, F, F, F) {
    let t7602 = t4998 * t2637;
    let t7603 = t2013 * t7602;
    let t7624 = t4419 * t2643;
    let t7625 = t782 * t7624;
    let t7632 = t5507 * t2642;
    let t7648 = t2618 * t1993;
    let t7656 = t2656 * t2041;
    (t7602, t7603, t7624, t7625, t7632, t7648, t7656)
}
