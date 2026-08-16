//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1125/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1125<F: Float>(t25352: F, t689: F, t7049: F, t786: F, t789: F, t1956: F, t213: F, t25287: F, t25292: F, t25297: F, t25303: F, t25307: F, t25311: F, t25314: F, t25319: F, t25322: F, t25326: F, t25333: F, t25337: F, t25340: F, t25344: F, t25349: F, t257: F, t2772: F, t7053: F, t7067: F, t7070: F, t7083: F, t887: F) -> (F, F) {
    let t25353 = t689 * t25352;
    let t25355 = t786 * t7049;
    let t25356 = t25355 * t789;
    let t25360 = F::cast_from(0.65854491829355115987e0_f64) * t213 * t25287 * t257 + F::cast_from(0.17347256376410398924e1_f64) * t7070 * t25292 + F::cast_from(0.14456046980341999104e-1_f64) * t25297 + t25303 - t25307 + F::cast_from(0.14456046980341999104e-1_f64) * t25311 - F::cast_from(0.4336814094102599731e0_f64) * t1956 * t25314 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25319 - F::cast_from(0.13170898365871023197e1_f64) * t25322 * t887 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t25326 + F::cast_from(0.13170898365871023197e1_f64) * t7053 * t2772 + t25333 - t25337 - F::cast_from(0.10975748638225852664e-1_f64) * t25340 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t25344 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t25349 + F::cast_from(0.10975748638225852664e-1_f64) * t25353 + F::cast_from(0.19514881078765566038e-1_f64) * t25356 - F::cast_from(0.8673628188205199462e0_f64) * t7067 * t7083;
    (t25355, t25360)
}
