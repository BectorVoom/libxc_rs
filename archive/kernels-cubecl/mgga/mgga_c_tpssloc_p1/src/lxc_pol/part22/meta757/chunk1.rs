//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2543/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2543<F: Float>(t18730: F, t4764: F, t21801: F, t699: F, t21788: F, t21791: F, t1113: F, t136: F, t71177: F, t3297: F, t71181: F, t71185: F) -> (F, F, F, F, F, F, F) {
    let t71468 = t4764 * t18730;
    let t71470 = t699 * t21801;
    let t71472 = t699 * t21788;
    let t71474 = t699 * t21791;
    let t71477 = t136 * t1113 * t71177;
    let t71480 = t136 * t3297 * t71181;
    let t71483 = t136 * t3297 * t71185;
    (t71468, t71470, t71472, t71474, t71477, t71480, t71483)
}
