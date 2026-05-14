//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 372/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk372<F: Float>(t1707: F, t2408: F, t1714: F, t1248: F, t1720: F, t2364: F, t1712: F, t1719: F, t2402: F, t1725: F) -> (F, F, F, F, F) {
    let t2409 = t1707 * t2408;
    let t2412 = t1714 * t2408;
    let t2415 = t1248 * t1720 * t2364;
    let t2417 = 0.1898925e1 * t2409 - t1712 - 0.29896666666666666667e0 * t2402 + 0.3071625e0 * t2412 - t1719 - 0.16431333333333333333e0 * t2415;
    let t2418 = t2417 * t1725;
    (t2409, t2412, t2415, t2417, t2418)
}
