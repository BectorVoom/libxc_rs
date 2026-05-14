//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 922/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk922<F: Float>(t2179: F, t86597: F, t4724: F, t39653: F, t4805: F, t9439: F, t4753: F, t40280: F, t91: F, t4462: F, t4714: F, t1017: F, t20027: F, t4668: F, t4454: F, t4458: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t86598 = t2179 * t86597;
    let t86600 = t4724 * t4724;
    let t86601 = t39653 * t86600;
    let t86604 = t9439 * t4724 * t4805;
    let t86606 = t4753 * t4753;
    let t86608 = t91 * t40280 * t86606;
    let t86610 = t4462 * t4714;
    let t86614 = t20027 * t1017;
    let t86618 = t4462 * t4668;
    let t86622 = t4454 * t4714;
    let t86626 = t4454 * t4668;
    let t86630 = t4458 * t4714;
    (t86598, t86601, t86604, t86608, t86610, t86614, t86618, t86622, t86626, t86630)
}
