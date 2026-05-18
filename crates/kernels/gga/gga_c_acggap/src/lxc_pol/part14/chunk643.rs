//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 643/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk643<F: Float>(t1750: F, t3379: F, t1165: F, t1748: F, t4289: F, t1298: F, t157: F, t1532: F, t1410: F, t944: F, t1173: F, t1180: F, t1531: F, t418: F, t5007: F, t5017: F, t5086: F, t5090: F, t5092: F, t5102: F, t5104: F, t5126: F, t5131: F, t5135: F, t5149: F, t6228: F, t6237: F, t6241: F, t6245: F, t6249: F) -> (F, F, F, F, F) {
    let t6252 = t3379 * t1750;
    let t6255 = t1165 * t4289 * t1748;
    let t6258 = t157 * t1298;
    let t6260 = t1165 * t1532 * t6258;
    let t6263 = t944 * t1410;
    let t6265 = t1165 * t1532 * t6263;
    let t6268 = -F::new(0.85748036236139473944e-3) * t418 * t6228 - t5007 + F::new(0.80031500487063509016e-2) * t5017 - t5086 - F::new(0.85748036236139473944e-3) * t5090 - F::new(0.42874018118069736972e-3) * t5092 - F::new(0.17149607247227894789e-2) * t5102 - F::new(0.80031500487063509015e-2) * t5104 + F::new(0.85748036236139473944e-3) * t1173 * t6237 - F::new(0.42874018118069736972e-3) * t1180 * t6241 + F::new(0.42874018118069736972e-3) * t1180 * t6245 - F::new(0.42874018118069736972e-3) * t1180 * t6249 + F::new(0.17149607247227894789e-2) * t6252 + F::new(0.17149607247227894789e-2) * t1173 * t6255 + F::new(0.17149607247227894789e-2) * t1173 * t6260 + F::new(0.85748036236139473944e-3) * t1531 * t6265 + t5126 - t5131 + t5135 - t5149;
    (t6255, t6260, t6263, t6265, t6268)
}
