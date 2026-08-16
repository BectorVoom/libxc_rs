//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2538/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2538<F: Float>(t136: F, t43761: F, t71164: F, t1100: F, t71390: F, t1113: F, t71148: F, t21794: F, t699: F, t11219: F, t71158: F, t71133: F) -> (F, F, F, F, F, F) {
    let t71400 = t136 * t43761 * t71164;
    let t71403 = t1100 * t71390;
    let t71406 = t136 * t1113 * t71148;
    let t71408 = t699 * t21794;
    let t71411 = t136 * t11219 * t71158;
    let t71414 = t136 * t11219 * t71133;
    (t71400, t71403, t71406, t71408, t71411, t71414)
}
