//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1022/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1022<F: Float>(t16013: F, t7370: F, t10459: F, t702: F, t15999: F, t172: F, t41: F, t139: F, t16009: F, t11830: F, t16854: F, t16858: F, t16888: F, t16893: F, t16926: F, t16930: F, t18042: F, t18045: F, t18054: F, t18057: F, t18058: F, t18063: F, t18070: F, t5231: F, t6278: F, t7030: F, t709: F, t725: F, t7349: F, t7360: F) -> (F, F) {
    let t18073 = t7370 * t16013;
    let t18076 = t10459 * t702;
    let t18077 = t18076 * t15999;
    let t18080 = t172 * t41;
    let t18081 = t139 * t18080;
    let t18082 = t7370 * t16009;
    let t18087 = 0.619125e-2 * t18042 * t709 + 0.9286875e-2 * t18045 * t7030 + 0.9286875e-2 * t725 * t16854 + 0.1857375e-1 * t5231 * t16893 + 0.70749629629629629629e-1 * t18054 - t18057 + 0.371475e-1 * t5231 * t18058 - 0.46434375e-2 * t7349 * t16926 - 0.232171875e-2 * t18063 * t16888 - 0.29479012345679012345e-1 * t11830 + 0.24765e-1 * t7360 * t16858 - 0.88437037037037037036e-1 * t6278 * t18070 - 0.44218518518518518518e-1 * t6278 * t18073 - 0.11791604938271604938e0 * t6278 * t18077 + 0.17687407407407407407e0 * t18081 * t18082 - 0.371475e-1 * t7360 * t16930;
    (t18081, t18087)
}
