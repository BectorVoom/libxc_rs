//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1054/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1054<F: Float>(t21105: F, t339: F, t4867: F, t850: F, t851: F, t860: F, t6588: F, t899: F, t900: F, t907: F, t6593: F, t855: F, t859: F, t854: F, t19553: F, t858: F, t884: F, t886: F) -> (F, F, F, F, F) {
    let t21106 = param_a_c * t21105;
    let t21111 = t4867 * t339;
    let t21115 = t850 * t851 * t21111 * t860 / 96.0;
    let t21117 = t899 * t900 * t6588;
    let t21118 = t21117 * t907;
    let t21121 = t855 * t6593 * t859;
    let t21122 = t854 * t21121;
    let t21123 = 455.0 / 324.0 * t21122;
    let t21127 = t884 * t886 * t858 * t19553 / 48.0;
    (t21106, t21115, t21118, t21123, t21127)
}
