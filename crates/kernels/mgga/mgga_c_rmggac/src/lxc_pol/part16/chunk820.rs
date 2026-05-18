//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 820/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk820<F: Float>(t1550: F, t40331: F, t2289: F, t7939: F, t2323: F, t638: F, t7184: F, t2004: F, t9090: F, t2007: F, t1987: F, t1990: F) -> (F, F, F, F, F, F, F) {
    let t40332 = t1550 * t40331;
    let t40339 = t7939 * t2289;
    let t40343 = t638 * t7184 * t2323;
    let t40349 = t9090 * t2004;
    let t40351 = t9090 * t2007;
    let t40354 = t9090 * t1987;
    let t40356 = t9090 * t1990;
    (t40332, t40339, t40343, t40349, t40351, t40354, t40356)
}
