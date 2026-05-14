//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1391/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1391<F: Float>(t111206: F, t111221: F, t111223: F, t111224: F, t111472: F, t116002: F, t116014: F, t15824: F, t1628: F, t2053: F, t2776: F, t32870: F, t6650: F, t7694: F, t806: F, t9904: F) -> (F,) {
    let t118600 = t9904 * t32870 / 16.0 + t111206 + t116002 - t2776 * t1628 * t7694 / 8.0 - t2776 * t6650 * t2053 / 8.0 - t111221 - t111223 - t111224 - t2776 * t15824 * t806 / 16.0 + t111472 + t116014;
    (t118600,)
}
