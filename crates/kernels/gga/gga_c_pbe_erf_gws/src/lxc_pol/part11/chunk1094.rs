//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1094/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1094<F: Float>(t13126: F, t3786: F, t4395: F, t860: F, t13293: F, t36962: F, t45584: F, t28269: F, t3065: F, t49794: F, t858: F, t45133: F, t9016: F, t44650: F, t1105: F, t3854: F) -> (F, F, F, F, F, F, F) {
    let t49819 = t13126 * t4395 * t3786 * t860 / 16.0;
    let t49826 = 11.0 / 96.0 * t36962 * t13293;
    let t49828 = 7.0 / 4.0 * t45584;
    let t49832 = t28269 * t3065 * t858 * t49794 / 8.0;
    let t49837 = t9016 * t45133 / 4.0;
    let t49839 = t9016 * t44650 / 8.0;
    let t49841 = t1105 * t3854;
    (t49819, t49826, t49828, t49832, t49837, t49839, t49841)
}
