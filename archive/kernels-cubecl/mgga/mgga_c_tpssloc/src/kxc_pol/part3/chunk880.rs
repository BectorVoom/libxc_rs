//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 880/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk880<F: Float>(t2617: F, t2642: F, t1891: F, t67: F, t246: F, t232: F, t2379: F, t2628: F, t835: F, t812: F, t2635: F, t2690: F, t815: F) -> (F, F, F, F, F, F) {
    let t9642 = t2617 * t2642;
    let t9645 = t1891 * t67;
    let t9646 = t9645 * t246;
    let t9647 = t232 * t2379;
    let t9666 = t2628 * t835;
    let t9667 = t812 * t9666;
    let t9668 = t9667 * t2635;
    let t9670 = t815 * t2690;
    (t9642, t9645, t9646, t9647, t9668, t9670)
}
