//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1011/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1011<F: Float>(t6241: F, t6394: F, t14287: F, t8189: F, t1520: F, t14294: F, t2282: F, t4170: F, t4165: F, t8286: F, t25365: F, t4204: F, t4203: F, t2258: F, t469: F, t6318: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t27054 = 2.0 * t6241 * t6394;
    let t27056 = 2.0 * t14287 * t8189;
    let t27057 = t8189 * t1520;
    let t27059 = 6.0 * t14294 * t27057;
    let t27060 = t2282 * t6394;
    let t27062 = 4.0 * t4170 * t27060;
    let t27063 = t4165 * t8286;
    let t27064 = t8286 * t1520;
    let t27066 = 2.0 * t4170 * t27064;
    let t27067 = t4204 * t25365;
    let t27068 = t4203 * t27067;
    let t27070 = t2258 * t469;
    let t27071 = t27070 * t6318;
    (t27054, t27056, t27057, t27059, t27060, t27062, t27063, t27064, t27066, t27067, t27068, t27071)
}
