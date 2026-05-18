//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 957/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk957<F: Float>(t10778: F, t1758: F, t11: F, t10788: F, t2704: F, t10792: F, t571: F, t10796: F, t10443: F, t10438: F, t3422: F, t395: F) -> (F, F, F, F, F, F, F) {
    let t10803 = t1758 * t10778;
    let t10804 = t11 * t10803;
    let t10806 = t1758 * t10788;
    let t10807 = t2704 * t10806;
    let t10809 = t571 * t10792;
    let t10810 = t11 * t10809;
    let t10812 = t571 * t10796;
    let t10813 = t2704 * t10812;
    let t10815 = t1758 * t10443;
    let t10816 = t11 * t10815;
    let t10818 = t571 * t10438;
    let t10819 = t11 * t10818;
    let t10823 = t395 * t3422;
    (t10804, t10807, t10810, t10813, t10816, t10819, t10823)
}
