//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 319/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk319<F: Float>(t1331: F, t1345: F, t1347: F, t1355: F, t1360: F, t1367: F, t241: F, t252: F, t810: F, t829: F, t1366: F, t828: F, t837: F, t845: F, t1235: F, t865: F) -> (F, F, F, F, F) {
    let t1371 = t241 * (-0.3109e-1 * t1347 * t252 + 1.0 * t810 * t1355 + t1331 - t1345 - 0.19751789702565206229e-1 * t1360 + 0.58482233974552040708e0 * t829 * t1367);
    let t1373 = 0.19751789702565206229e-1 * t241 * t1360;
    let t1375 = t828 * t1366 * t837;
    let t1377 = 0.58482233974552040708e0 * t845 * t1375;
    let t1378 = t865 * t1235;
    (t1371, t1373, t1375, t1377, t1378)
}
