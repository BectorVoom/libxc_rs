//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 631/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk631<F: Float>(t1778: F, t1787: F, t4987: F, t4997: F, t5000: F, t664: F, t7203: F, t7206: F, t7208: F, t7213: F, t7216: F, t7219: F, t2459: F, t4998: F, t1773: F, t5005: F, t9: F) -> (F, F, F, F) {
    let t7226 = 0.5397236614853195164e-1 * t7203 * t664 + 0.17990788716177317213e-1 * t7206 + 0.17990788716177317213e-1 * t7208 * t1778 - 0.5397236614853195164e-1 * t7208 * t1787 - 0.14392630972941853771e0 * t7213 * t664 - 0.47975436576472845903e-1 * t7216 - 0.47975436576472845903e-1 * t7219 * t1778 + 0.14392630972941853771e0 * t7219 * t1787 + 0.17990788716177317213e-1 * t4987 - t4997 + 0.59969295720591057377e-2 * t5000;
    let t7230 = t4998 * t2459;
    let t7231 = t1773 * t7230;
    let t7233 = t9 * t5005;
    (t7226, t7230, t7231, t7233)
}
