//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1283/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1283<F: Float>(t1167: F, t218: F, t219: F, t9795: F, t11153: F, t824: F, t31086: F, t334: F, t11205: F, t675: F, t11209: F, t3747: F, t836: F) -> (F, F, F, F, F, F) {
    let t31254 = t218 * t219 * t1167 * t9795;
    let t31258 = t218 * t219 * t824 * t11153;
    let t31262 = t218 * t219 * t334 * t31086;
    let t31265 = t218 * t675 * t11205;
    let t31268 = t218 * t675 * t11209;
    let t31270 = t3747 * t836;
    (t31254, t31258, t31262, t31265, t31268, t31270)
}
