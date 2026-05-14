//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 564/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk564<F: Float>(t106: F, t3232: F, t797: F, t97: F, t1356: F, t1360: F, t1387: F, t1413: F, t2322: F, t2460: F, t2891: F, t2895: F, t2896: F, t2897: F, t2997: F, t2998: F, t3019: F, t3128: F, t3162: F, t3165: F, t372: F, t881: F) -> (F,) {
    let t3235 = t97 * t106 * t3232 * t797;
    let t3243 = 2.0 * t2460 - t2891 + t1356 + t1360 - t2895 - t2896 + t2897 + t2997 + t2998 + t1387 + t1413 + t2322 - 0.2363e1 * t881 * t3162 - 0.4726e1 * t881 * t3165 + t372 * t3128 - t3019 - t3235;
    (t3243,)
}
