//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 692/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk692<F: Float>(t1562: F, t2538: F, t285: F, t3053: F, t3056: F, t3060: F, t3229: F, t499: F, t921: F, t106: F, t797: F, t97: F, t1356: F, t1360: F, t1387: F, t1413: F, t2322: F, t2460: F, t2891: F, t2895: F, t2896: F, t2897: F, t2997: F, t2998: F, t3019: F, t3128: F, t3162: F, t3165: F, t372: F, t881: F) -> (F, F) {
    let t3232 = t3053 * t285 + t3056 * t285 + t921 * t2538 / 2.0 - 5.0 / 16.0 * t1562 * t3060 + t499 * t3229 / 4.0;
    let t3235 = t97 * t106 * t3232 * t797;
    let t3243 = 2.0 * t2460 - t2891 + t1356 + t1360 - t2895 - t2896 + t2897 + t2997 + t2998 + t1387 + t1413 + t2322 - 0.2363e1 * t881 * t3162 - 0.4726e1 * t881 * t3165 + t372 * t3128 - t3019 - t3235;
    (t3232, t3243)
}
