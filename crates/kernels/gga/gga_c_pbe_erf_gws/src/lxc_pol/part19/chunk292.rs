//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 292/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk292<F: Float>(t858: F, t875: F, t867: F, t866: F, t338: F, t348: F, t839: F, t331: F, t855: F, t863: F) -> (F, F, F, F, F) {
    let t876 = t858 * t875;
    let t877 = t867 * t876;
    let t879 = t866 * t877 / 96.0;
    let t882 = 7.0 / 288.0 * t348 * t839 * t338;
    let t884 = t863 * t855 * t331;
    (t876, t877, t879, t882, t884)
}
