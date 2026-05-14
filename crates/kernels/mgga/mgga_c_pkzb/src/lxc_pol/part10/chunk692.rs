//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 692/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk692<F: Float>(t3033: F, t853: F, t1185: F, t2192: F, t851: F, t2197: F, t1174: F, t2203: F, t836: F, t2175: F, t2207: F, t3017: F, t3028: F) -> (F, F, F, F, F, F, F) {
    let t3035 = 1.0 * t3033 * t853;
    let t3037 = 1.0 * t2192 * t1185;
    let t3038 = t1185 * t851;
    let t3040 = 2.0 * t2197 * t3038;
    let t3041 = t2203 * t1174;
    let t3042 = t3041 * t836;
    let t3046 = t2207 - t2175 / 3.0 - t3017 / 3.0 + t3028;
    (t3035, t3037, t3038, t3040, t3041, t3042, t3046)
}
