//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 672/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk672<F: Float>(t241: F, t2719: F, t820: F, t243: F, t72: F, t245: F, t2723: F, t836: F, t162: F, t2611: F, t227: F, t73: F, t225: F, t2718: F, t213: F, t2783: F) -> (F, F, F, F, F, F, F, F) {
    let t4362 = t820 * t2719 * t241;
    let t4363 = t243 * t72;
    let t4364 = t4363 * t245;
    let t4366 = t2723 * t836;
    let t4401 = t2611 * t162;
    let t4415 = t227 * t73;
    let t4503 = t225 * t2718;
    let t4504 = t213 * t4503;
    let t4514 = t213 * t2783;
    (t4362, t4364, t4366, t4401, t4415, t4503, t4504, t4514)
}
