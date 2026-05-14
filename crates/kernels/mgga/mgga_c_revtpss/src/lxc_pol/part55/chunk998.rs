//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 998/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk998<F: Float>(t11064: F, t7427: F, t1892: F, t7063: F, t25081: F, t7897: F, t7234: F, t8995: F, t2: F, t2411: F, t1468: F, t605: F, t30: F, t41154: F, t1568: F, t1113: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t95976 = t7427 * t11064;
    let t98040 = t7063 * t1892;
    let t98450 = t7897 * t25081;
    let t98588 = t7234 * t8995;
    let t98631 = t2411 * t2;
    let t98658 = t2411 * t1468;
    let t98763 = t11064 * t605;
    let t98785 = t41154 * t30;
    let t98848 = t7063 * t1568;
    let t100974 = t11064 * t1113;
    (t95976, t98040, t98450, t98588, t98631, t98658, t98763, t98785, t98848, t100974)
}
