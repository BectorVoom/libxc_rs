//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1433/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1433<F: Float>(t12898: F, t1786: F, t13041: F, t56730: F, t11772: F, t17394: F, t3717: F, t12865: F, t17400: F, t1222: F, t1781: F, t2438: F) -> (F, F, F, F, F) {
    let t57615 = t1786 * t12898;
    let t57641 = t56730 * t13041;
    let t57659 = t17394 * t11772;
    let t57660 = t3717 * t57659;
    let t57663 = t17400 * t12865;
    let t57687 = t1222 * t2438 * t1781;
    (t57615, t57641, t57660, t57663, t57687)
}
