//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 482/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk482(t1152: f64, t1157: f64, t1392: f64, t1430: f64, t1442: f64, t1835: f64, t1839: f64, t198: f64, t4382: f64, t4389: f64, t446: f64, t454: f64, t5477: f64, t5480: f64, t589: f64, t6017: f64, t6020: f64, t6031: f64, t6034: f64, t6039: f64, t6067: f64) -> f64 {
    let t6070 = -0.32163648644302209643e2_f64 * t6017 * t198 + 0.96490945932906628929e2_f64 * t6020 * t446 + 0.19298189186581325786e3_f64 * t5477 * t589 - 0.77192756746325303144e3_f64 * t5480 * t1430 + 0.19298189186581325786e3_f64 * t1442 * t1392 - 0.38596378373162651572e3_f64 * t4382 * t1839 + 0.19298189186581325786e4_f64 * t4389 * t6031 - 0.77192756746325303144e3_f64 * t1157 * t6034 + 0.96490945932906628929e2_f64 * t1152 * t1835 - 0.38596378373162651572e3_f64 * t1157 * t6039 + 0.96490945932906628929e2_f64 * t454 * t6067;
    t6070
}
