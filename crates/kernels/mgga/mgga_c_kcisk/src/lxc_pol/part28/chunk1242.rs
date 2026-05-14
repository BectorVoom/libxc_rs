//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1242/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1242<F: Float>(t34329: F, t9972: F, t9047: F, t9704: F, t1800: F, t9066: F, t9086: F, t7316: F, t8672: F, t33097: F, t5062: F, t9051: F, t733: F, t9055: F, t35283: F, t35285: F, t35287: F, t35289: F, t35291: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35293 = t34329 * t9972;
    let t35295 = t9704 * t9047;
    let t35297 = t1800 * t9066;
    let t35299 = t1800 * t9086;
    let t35301 = t7316 * t8672;
    let t35302 = t33097 * t35301;
    let t35304 = t5062 * t9051;
    let t35306 = t733 * t9055;
    let t35308 = t35283 / 64.0 + t35285 / 12.0 - t35287 / 48.0 - t35289 / 12.0 + t35291 / 48.0 + t35293 / 3.0 - t35295 / 12.0 - t35297 / 288.0 - t35299 / 96.0 + t35302 / 8.0 - t35304 / 64.0 + 11.0 / 18.0 * t35306;
    (t35293, t35295, t35297, t35299, t35301, t35302, t35304, t35306, t35308)
}
