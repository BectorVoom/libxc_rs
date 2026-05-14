//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 524/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk524<F: Float>(t2039: F, t638: F, t7385: F, t303: F, t31: F, t2046: F, t2050: F, t357: F, t1990: F, t2186: F, t1271: F, t1986: F, t675: F, t4443: F, t671: F, t674: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7387 = t638 * t2039 * t7385;
    let t7389 = t303 * t31;
    let t7391 = t2046 * t2050 * t7389;
    let t7393 = t357 * t31;
    let t7395 = t2046 * t2050 * t7393;
    let t7402 = t2186 * t1990;
    let t7404 = t1986 * t1271;
    let t7405 = t675 * t7404;
    let t7407 = t671 * t4443;
    let t7408 = t7407 * t674;
    (t7387, t7389, t7391, t7393, t7395, t7402, t7404, t7405, t7407, t7408)
}
