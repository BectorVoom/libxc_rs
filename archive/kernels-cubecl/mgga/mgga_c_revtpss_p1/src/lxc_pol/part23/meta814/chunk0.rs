//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2659/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2659<F: Float>(t11120: F, t1651: F, t1071: F, t19462: F, t19856: F, t378: F, t1647: F, t4930: F, t3056: F, t6234: F, t15669: F, t379: F) -> (F, F, F, F, F, F, F) {
    let t64614 = t11120 * t1651;
    let t64629 = t19462 * t1071;
    let t64636 = t19856 * t378;
    let t64639 = t1647 * t4930;
    let t64686 = t6234 * t3056;
    let t64687 = t64686 * t378;
    let t64711 = t15669 * t379;
    (t64614, t64629, t64636, t64639, t64686, t64687, t64711)
}
