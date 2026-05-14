//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1063/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1063<F: Float>(t22206: F, t22221: F, t44: F, t1755: F, t2527: F, t1785: F, t2464: F, t2023: F, t2642: F, t167: F, t3233: F, t3236: F, t9345: F, t1032: F, t967: F, t3139: F) -> (F, F, F, F, F, F, F, F) {
    let t22223 = (t22206 + t22221) * t44;
    let t22999 = t1755 * t2527;
    let t23819 = t2464 * t1785;
    let t24991 = t2642 * t2023;
    let t31825 = t3233 * t167;
    let t31827 = t3236 * t9345;
    let t31829 = t1032 * t967;
    let t31831 = t167 * t3139;
    (t22223, t22999, t23819, t24991, t31825, t31827, t31829, t31831)
}
