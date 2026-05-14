//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1068/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1068<F: Float>(t24315: F, t24361: F, t24403: F, t24442: F, t716: F, t736: F, t23275: F, t740: F, t748: F, t1945: F, t9079: F, t9082: F, t22278: F, t5322: F, t7429: F, t1931: F, t9055: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24444 = t24315 + t24361 + t24403 + t24442;
    let t24445 = t24444 * t716;
    let t24446 = t24445 * sigma2;
    let t24447 = t24446 * t736;
    let t24449 = t23275 * t716;
    let t24450 = t24449 * t740;
    let t24451 = t24450 * t748;
    let t24453 = t1945 * t9079;
    let t24455 = t1945 * t9082;
    let t24457 = t5322 * t22278;
    let t24458 = t7429 * t24457;
    let t24460 = t1931 * t9055;
    (t24445, t24447, t24449, t24451, t24453, t24455, t24457, t24458, t24460)
}
