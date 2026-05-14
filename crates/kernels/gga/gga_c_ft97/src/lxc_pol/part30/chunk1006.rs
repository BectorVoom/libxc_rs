//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1006/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1006<F: Float>(t1466: F, t35794: F, t681: F, t35801: F, t36063: F, t44351: F, t1476: F, t7129: F, t143017: F, t6967: F, t1506: F, t7021: F, t1253: F, t7584: F, t36093: F, t6213: F) -> (F, F, F, F, F, F, F, F) {
    let t153681 = t1466 * t681 * t35794;
    let t153684 = t1466 * t681 * t35801;
    let t153687 = t44351 * t36063;
    let t153689 = t1476 * t7129;
    let t153696 = t143017 * t6967;
    let t153698 = t7021 * t1506;
    let t153705 = t7584 * t1253;
    let t153710 = t36093 * t6213;
    (t153681, t153684, t153687, t153689, t153696, t153698, t153705, t153710)
}
