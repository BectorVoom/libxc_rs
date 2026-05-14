//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 422/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk422<F: Float>(t240: F, t4719: F, t1302: F, t255: F, t1309: F, t16: F, t4709: F, t1297: F, t1314: F, t1310: F, t20: F, t252: F, t43: F, t1303: F, t239: F, t1311: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4720 = t240 * t4719;
    let t4724 = t255 * t1302;
    let t4728 = 1.0 / t1309 / t16;
    let t4729 = t4728 * t4709;
    let t4732 = t1297 * t1314;
    let t4737 = t1310 * t1302;
    let t4738 = t252 * t20;
    let t4739 = t4738 * t43;
    let t4742 = t1303 * t1314;
    let t4746 = 1.0 / t1309 / t239;
    let t4747 = t4746 * t4709;
    let t4750 = t1311 * t1314;
    (t4720, t4724, t4729, t4732, t4737, t4739, t4742, t4747, t4750)
}
