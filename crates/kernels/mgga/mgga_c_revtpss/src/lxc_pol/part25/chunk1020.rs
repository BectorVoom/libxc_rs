//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1020/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1020<F: Float>(t25348: F, t7076: F, t7014: F, t887: F, t689: F, t7049: F, t786: F, t789: F, t1956: F, t213: F, t25287: F, t25292: F, t25297: F, t25303: F, t25307: F, t25311: F, t25314: F, t25319: F, t25322: F, t25326: F, t25333: F, t25337: F, t25340: F, t25344: F, t257: F, t2772: F, t7053: F, t7067: F, t7070: F, t7083: F) -> (F, F, F, F) {
    let t25349 = t7076 * t25348;
    let t25352 = t7014 * t887;
    let t25353 = t689 * t25352;
    let t25355 = t786 * t7049;
    let t25356 = t25355 * t789;
    let t25360 = 0.65854491829355115987e0 * t213 * t25287 * t257 + 0.17347256376410398924e1 * t7070 * t25292 + 0.14456046980341999104e-1 * t25297 + t25303 - t25307 + 0.14456046980341999104e-1 * t25311 - 0.4336814094102599731e0 * t1956 * t25314 - 0.26020884564615598386e1 * t7070 * t25319 - 0.13170898365871023197e1 * t25322 * t887 + 0.8673628188205199462e0 * t7070 * t25326 + 0.13170898365871023197e1 * t7053 * t2772 + t25333 - t25337 - 0.10975748638225852664e-1 * t25340 + 0.8673628188205199462e0 * t7070 * t25344 + 0.4336814094102599731e0 * t7070 * t25349 + 0.10975748638225852664e-1 * t25353 + 0.19514881078765566038e-1 * t25356 - 0.8673628188205199462e0 * t7067 * t7083;
    (t25349, t25352, t25355, t25360)
}
