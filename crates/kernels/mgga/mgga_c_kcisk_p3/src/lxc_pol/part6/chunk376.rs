//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 376/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk376<F: Float>(t1707: F, t2408: F, t1714: F, t1248: F, t1720: F, t2364: F, t1712: F, t1719: F, t2402: F, t1725: F, t1729: F, t1739: F, t1742: F) -> (F, F, F, F) {
    let t2409 = t1707 * t2408;
    let t2412 = t1714 * t2408;
    let t2415 = t1248 * t1720 * t2364;
    let t2417 = F::new(0.1898925e1) * t2409 - t1712 - F::cast_from(0.29896666666666666667e0_f64) * t2402 + F::new(0.3071625e0) * t2412 - t1719 - F::cast_from(0.16431333333333333333e0_f64) * t2415;
    let t2418 = t2417 * t1725;
    let t2422 = -t1729 - F::cast_from(0.92708333333333333333e-2_f64) * t2402;
    let t2430 = F::new(0.258925e1) * t2409 - t1739 - F::new(0.301925e0) * t2402 + F::new(0.16504875e0) * t2412 - t1742 - F::new(0.16557e0) * t2415;
    (t2417, t2418, t2422, t2430)
}
