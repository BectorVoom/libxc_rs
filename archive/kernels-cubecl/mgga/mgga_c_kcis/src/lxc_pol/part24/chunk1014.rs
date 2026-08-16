//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1014/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1014<F: Float>(t26611: F, t7580: F, t209: F, t2410: F, t7590: F, t7589: F, t698: F, t2389: F, t700: F, t705: F) -> (F, F, F, F, F, F) {
    let t26612 = t7580 * t26611;
    let t26615 = t209 * t7590 * t2410;
    let t26616 = t7589 * t26615;
    let t26618 = t7589 * t26611;
    let t26620 = t209 * t698;
    let t26623 = t26620 * t2389 * t700 * t705;
    (t26612, t26615, t26616, t26618, t26620, t26623)
}
