//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1241/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1241<F: Float>(t1248: F, t6573: F, t1250: F, t3720: F, t19666: F, t5302: F, t1042: F, t17550: F, t19661: F, t1715: F, t17500: F, t5056: F, t5277: F, t20261: F, t20263: F, t20386: F, t20388: F, t20390: F, t20393: F, t20396: F, t20399: F, t20402: F, t20404: F, t20450: F, t20452: F, t20454: F, t20471: F, t20475: F, t20477: F, t20685: F) -> (F, F, F, F, F, F, F) {
    let t20856 = t6573 * t1248;
    let t20857 = t20856 * t1250;
    let t20858 = t3720 * t20857;
    let t20863 = t5302 * t19666;
    let t20864 = t1042 * t20863;
    let t20867 = t17550 * t19661;
    let t20868 = t1042 * t20867;
    let t20875 = t17500 * t1715;
    let t20876 = t1042 * t20875;
    let t20879 = t5277 * t5056;
    let t20880 = t1042 * t20879;
    let t20885 = -t20261 - t20263 - t20386 - t20388 - t20390 - t20393 + t20396 - t20399 - t20402 - t20404 + t20450 + t20452 + t20454 - t20471 + t20475 + t20477 + t20685;
    (t20856, t20858, t20864, t20868, t20876, t20880, t20885)
}
