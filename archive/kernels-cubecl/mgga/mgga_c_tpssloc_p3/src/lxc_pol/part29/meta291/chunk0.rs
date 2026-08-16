//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1318/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1318<F: Float>(t246: F, t9645: F, t232: F, t2379: F, t2628: F, t835: F, t812: F, t2635: F, t2690: F, t815: F, t831: F, t2617: F, t2638: F) -> (F, F, F, F, F, F) {
    let t9646 = t9645 * t246;
    let t9647 = t232 * t2379;
    let t9666 = t2628 * t835;
    let t9667 = t812 * t9666;
    let t9668 = t9667 * t2635;
    let t9670 = t815 * t2690;
    let t9671 = t812 * t9670;
    let t9672 = t9671 * t831;
    let t9674 = t2617 * t2638;
    (t9646, t9647, t9668, t9671, t9672, t9674)
}
