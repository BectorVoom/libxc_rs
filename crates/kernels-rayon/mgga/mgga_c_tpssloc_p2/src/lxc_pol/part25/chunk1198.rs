//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1198/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1198(t12248: f64, t2085: f64, t12238: f64, t12251: f64, t12255: f64, t1332: f64, t1336: f64, t2089: f64, t24103: f64, t24117: f64, t24121: f64, t24127: f64, t3777: f64, t81187: f64, t81189: f64, t81193: f64, t81197: f64, t81209: f64, t81213: f64, t81216: f64, t81218: f64, t81222: f64, t81225: f64, t81230: f64, t81234: f64, t81238: f64) -> f64 {
    let t84627 = t12248 * t2085;
    let t84634 = t12238 * t2089 - 0.76763589786250567036e0_f64 * t81187 + 0.46058153871750340221e0_f64 * t81189 + 0.29608813203268075857e0_f64 * t81193 + 0.9869604401089358619e-1_f64 * t81197 - 0.9869604401089358619e-1_f64 * t81209 - 0.3289868133696452873e-1_f64 * t81213 + 0.49348022005446793095e-1_f64 * t81216 + 0.23029076935875170111e0_f64 * t81218 - 0.19739208802178717238e0_f64 * t81222 - 0.49348022005446793095e-1_f64 * t81225 + 3.0_f64 * t1332 * t24121 - 0.9869604401089358619e-1_f64 * t81230 + 0.19739208802178717238e0_f64 * t81234 + 0.9869604401089358619e-1_f64 * t81238 - 6.0_f64 * t3777 * t24117 - 3.0_f64 * t3777 * t24103 - 6.0_f64 * t1336 * t84627 * t12251 + 6.0_f64 * t1336 * t24127 * t12255;
    t84634
}
