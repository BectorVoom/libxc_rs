//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1146/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1146<F: Float>(t4165: F, t9509: F, t2732: F, t4171: F, t14294: F, t1520: F, t4170: F, t4321: F, t1340: F, t4185: F, t4182: F, t4306: F, t1415: F, t4189: F, t1299: F, t1486: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32237 = 2.0 * t4165 * t9509;
    let t32238 = t2732 * t4171;
    let t32240 = 6.0 * t14294 * t32238;
    let t32241 = t9509 * t1520;
    let t32243 = 4.0 * t4170 * t32241;
    let t32244 = t2732 * t4321;
    let t32246 = 2.0 * t4170 * t32244;
    let t32247 = t1340 * t4185;
    let t32249 = t1340 * t4182;
    let t32251 = t1340 * t4306;
    let t32253 = t1415 * t4189;
    let t32255 = t1486 * t1299;
    (t32237, t32238, t32240, t32241, t32243, t32244, t32246, t32247, t32249, t32251, t32253, t32255)
}
