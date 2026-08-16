//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1126/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1126<F: Float>(t15488: F, t3052: F, t357: F, t5229: F, t339: F, t454: F, t242: F, t3090: F, t5068: F, t1125: F, t5072: F, t1120: F, t5231: F) -> (F, F, F, F, F) {
    let t15489 = t3052 * t15488;
    let t15491 = t5229 * t357;
    let t15493 = t339 * t454 * t15491;
    let t15499 = t242 * t3090 * t5068;
    let t15500 = t1125 * t15499;
    let t15503 = t242 * t3090 * t5072;
    let t15504 = t1125 * t15503;
    let t15506 = t5231 * t1120;
    (t15489, t15493, t15500, t15504, t15506)
}
