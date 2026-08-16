//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1217/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1217(t102798: f64, t107413: f64, t107417: f64, t107431: f64, t107435: f64, t107439: f64, t1336: f64, t1814: f64, t1825: f64, t20490: f64, t20495: f64, t20554: f64, t20595: f64, t2089: f64, t24127: f64, t27097: f64, t29327: f64, t29339: f64, t29349: f64, t5234: f64, t6388: f64, t6415: f64, t6420: f64, t7208: f64, t84627: f64, t91078: f64, t91081: f64, t93798: f64, t97494: f64) -> f64 {
    let t107987 = -0.49348022005446793095e-1_f64 * t107413 - 3.0_f64 * t1336 * t27097 * t6420 + 0.29608813203268075857e0_f64 * t107417 + t20595 * t2089 - t1336 * t7208 * t20554 - 6.0_f64 * t5234 * t29349 + 6.0_f64 * t5234 * t29339 + 6.0_f64 * t1336 * t93798 * t6388 - 3.0_f64 * t1336 * t102798 * t1825 - 6.0_f64 * t1336 * t84627 * t20490 + 6.0_f64 * t1336 * t24127 * t20495 - 0.15626873635058151147e0_f64 * t91078 + 0.9869604401089358619e-1_f64 * t91081 + 0.49348022005446793095e-1_f64 * t97494 + 3.0_f64 * t1814 * t29327 - 0.39478417604357434476e0_f64 * t107431 - 3.0_f64 * t1336 * t27097 * t6415 - 0.9869604401089358619e-1_f64 * t107435 + 0.9869604401089358619e-1_f64 * t107439;
    t107987
}
