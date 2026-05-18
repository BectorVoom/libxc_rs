//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1110/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1110<F: Float>(t11064: F, t7427: F, t116: F, t28159: F, t1892: F, t7063: F, t25081: F, t7897: F, t7234: F, t8995: F, t2: F, t2411: F) -> (F, F, F, F, F, F) {
    let t95976 = t7427 * t11064;
    let t97622 = t28159 * t116;
    let t98040 = t7063 * t1892;
    let t98450 = t7897 * t25081;
    let t98588 = t7234 * t8995;
    let t98631 = t2411 * t2;
    (t95976, t97622, t98040, t98450, t98588, t98631)
}
