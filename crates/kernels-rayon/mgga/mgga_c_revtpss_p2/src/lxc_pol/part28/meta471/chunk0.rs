//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1784/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1784(t1949: f64, t231: f64, t2645: f64, t7076: f64, t7014: f64, t887: f64, t689: f64, t7049: f64, t786: f64, t789: f64, t1956: f64, t213: f64, t25287: f64, t25292: f64, t25297: f64, t25303: f64, t25307: f64, t25311: f64, t25314: f64, t25319: f64, t25322: f64, t25326: f64, t25333: f64, t25337: f64, t25340: f64, t25344: f64, t257: f64, t2772: f64, t7053: f64, t7067: f64, t7070: f64, t7083: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25348 = t1949 * t2645 * t231;
    let t25349 = t7076 * t25348;
    let t25352 = t7014 * t887;
    let t25353 = t689 * t25352;
    let t25355 = t786 * t7049;
    let t25356 = t25355 * t789;
    let t25360 = 0.65854491829355115987e0_f64 * t213 * t25287 * t257 + 0.17347256376410398924e1_f64 * t7070 * t25292 + 0.14456046980341999104e-1_f64 * t25297 + t25303 - t25307 + 0.14456046980341999104e-1_f64 * t25311 - 0.4336814094102599731e0_f64 * t1956 * t25314 - 0.26020884564615598386e1_f64 * t7070 * t25319 - 0.13170898365871023197e1_f64 * t25322 * t887 + 0.8673628188205199462e0_f64 * t7070 * t25326 + 0.13170898365871023197e1_f64 * t7053 * t2772 + t25333 - t25337 - 0.10975748638225852664e-1_f64 * t25340 + 0.8673628188205199462e0_f64 * t7070 * t25344 + 0.4336814094102599731e0_f64 * t7070 * t25349 + 0.10975748638225852664e-1_f64 * t25353 + 0.19514881078765566038e-1_f64 * t25356 - 0.8673628188205199462e0_f64 * t7067 * t7083;
    (t25348, t25349, t25352, t25353, t25355, t25356, t25360)
}
