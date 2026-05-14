//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 842/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk842<F: Float>(t21310: F, t482: F, t11536: F, t7002: F, t11539: F, t1354: F, t11388: F, t5619: F, t3918: F, t7019: F, t1578: F, t5595: F, t6114: F, t17771: F, t5618: F, t3944: F) -> (F, F, F, F, F, F, F) {
    let t21311 = t21310 * t482;
    let t21314 = t11536 * t7002;
    let t21315 = t11539 * t1354;
    let t21316 = t21314 * t21315;
    let t21319 = t11388 * t7002;
    let t21320 = t21319 * t5619;
    let t21323 = t3918 * t7019;
    let t21324 = t21323 * t1578;
    let t21327 = t5595 * t6114;
    let t21330 = t5618 * t17771;
    let t21333 = t3944 * t7019;
    (t21311, t21316, t21320, t21324, t21327, t21330, t21333)
}
