//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 819/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk819<F: Float>(t219: F, t5463: F, t1620: F, t1811: F, t16675: F, t2559: F, t587: F, t1627: F, t4898: F, t1815: F, t422: F, t5097: F, t626: F, t639: F, t1698: F, t1724: F) -> (F, F, F, F, F) {
    let t16904 = t5463 * t219;
    let t16906 = t1620 * t16904 * t1811;
    let t16907 = 64.0 / 135.0 * t16906;
    let t16910 = 16.0 / 3.0 * t587 * t2559 * t16675;
    let t16912 = 16.0 / 15.0 * t1627 * t4898;
    let t16917 = 16.0 / 45.0 * t639 * t1815 * t5097 * t626 * t422;
    let t16921 = 16.0 / 15.0 * t639 * t1815 * t1698 * t1724;
    (t16907, t16910, t16912, t16917, t16921)
}
