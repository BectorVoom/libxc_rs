//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1095/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1095<F: Float>(t49841: F, t8884: F, t3138: F, t4386: F, t3824: F, t20933: F, t21298: F, t858: F, t867: F, t6241: F, t6240: F, t3128: F, t44315: F, t11592: F, t13491: F, t2121: F, t337: F, t3772: F, t3791: F, t9119: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t49842 = t8884 * t49841;
    let t49845 = t3138 * t4386 * t49842 / 2.0;
    let t49847 = t3824 * t3824;
    let t49848 = t49847 * t20933;
    let t49852 = t21298 * t867 * t858 * t49848 / 4.0;
    let t49853 = t49847 * t6241;
    let t49857 = 3.0 / 8.0 * t6240 * t867 * t858 * t49853;
    let t49859 = t3128 * t44315 / 12.0;
    let t49861 = t11592 * t13491 / 32.0;
    let t49875 = t9119 * t2121 * t337 * t3791 * t3772 / 16.0;
    (t49842, t49845, t49847, t49848, t49852, t49853, t49857, t49859, t49861, t49875)
}
