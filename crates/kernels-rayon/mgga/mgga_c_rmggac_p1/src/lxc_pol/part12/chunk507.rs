//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 507/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk507(t1144: f64, t1152: f64, t1157: f64, t1392: f64, t1430: f64, t1442: f64, t198: f64, t4379: f64, t4382: f64, t4389: f64, t446: f64, t454: f64, t5436: f64, t5439: f64, t5474: f64, t5477: f64, t5480: f64, t5491: f64, t5527: f64, t589: f64, t998: f64) -> f64 {
    let t5530 = -0.32163648644302209643e2_f64 * t5474 * t198 + 0.19298189186581325786e3_f64 * t5477 * t446 - 0.38596378373162651572e3_f64 * t5480 * t1144 + 0.96490945932906628929e2_f64 * t1442 * t998 + 0.96490945932906628929e2_f64 * t4379 * t589 - 0.77192756746325303144e3_f64 * t4382 * t1430 + 0.19298189186581325786e3_f64 * t1152 * t1392 + 0.19298189186581325786e4_f64 * t4389 * t5491 - 0.77192756746325303144e3_f64 * t1157 * t5436 - 0.38596378373162651572e3_f64 * t1157 * t5439 + 0.96490945932906628929e2_f64 * t454 * t5527;
    t5530
}
