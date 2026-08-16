//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1060/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1060<F: Float>(t4882: F, t513: F, t1511: F, t5089: F, t5152: F, t1545: F, t1628: F, t1548: F, t16540: F, t4920: F, t541: F, t555: F) -> (F, F, F, F, F, F) {
    let t16613 = t4882 * t513;
    let t16615 = t1511 * t5089;
    let t16617 = t1511 * t5152;
    let t16619 = t1545 * t1628;
    let t16621 = t1548 * t1628;
    let t16626 = F::cast_from(0.14035736694323150897e2_f64) * t555 * t4920 * t16540 * t541;
    (t16613, t16615, t16617, t16619, t16621, t16626)
}
