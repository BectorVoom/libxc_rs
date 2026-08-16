//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 307/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk307(t1286: f64, t1450: f64, t1340: f64, t1411: f64, t1216: f64, t1220: f64, t1329: f64, t1335: f64, t1344: f64, t1408: f64, t1443: f64, t1448: f64, t412: f64) -> (f64, f64, f64, f64) {
    let t1451 = t1450 * t1286;
    let t1452 = t1340 * t1451;
    let t1453 = t1411 * t1452;
    let t1455 = t1216 * t412 - 0.193e0_f64 * t1220 * t1329 + t1335 + 0.16581944444444444444e-2_f64 * t1344 + 0.24872916666666666666e-2_f64 * t1408 - 0.24872916666666666666e-2_f64 * t1443 - 0.66327777777777777776e-2_f64 * t1448 + 0.16581944444444444444e-2_f64 * t1453;
    (t1451, t1452, t1453, t1455)
}
