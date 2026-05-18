//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 307/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk307<F: Float>(t1286: F, t1450: F, t1340: F, t1411: F, t1216: F, t1220: F, t1329: F, t1335: F, t1344: F, t1408: F, t1443: F, t1448: F, t412: F) -> (F, F, F, F) {
    let t1451 = t1450 * t1286;
    let t1452 = t1340 * t1451;
    let t1453 = t1411 * t1452;
    let t1455 = t1216 * t412 - F::new(0.193e0) * t1220 * t1329 + t1335 + F::new(0.16581944444444444444e-2) * t1344 + F::new(0.24872916666666666666e-2) * t1408 - F::new(0.24872916666666666666e-2) * t1443 - F::new(0.66327777777777777776e-2) * t1448 + F::new(0.16581944444444444444e-2) * t1453;
    (t1451, t1452, t1453, t1455)
}
