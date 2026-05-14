//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 763/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk763<F: Float>(t481: F, t494: F, t7338: F, t7337: F, t560: F, t5109: F, t1593: F, t921: F, t2533: F, t2551: F, t7321: F, t2294: F, t2568: F, t2139: F, t2122: F, t2187: F, t5098: F, t5101: F, t5106: F, t5108: F, t6106: F, t6132: F, t6139: F, t6293: F, t6583: F, t7312: F, t7313: F, t7317: F, t7323: F, t7327: F, t7330: F, t7334: F) -> (F, F, F, F, F) {
    let t7339 = t494 * t481;
    let t7340 = t7338 * t7339;
    let t7341 = t7337 * t7340;
    let t7344 = t494 * t560;
    let t7345 = t7338 * t7344;
    let t7346 = t5109 * t7345;
    let t7349 = t5109 * t7340;
    let t7352 = t921 * t1593;
    let t7353 = t5109 * t7352;
    let t7356 = t2533 * t2551;
    let t7357 = t7321 * t7356;
    let t7360 = t2294 * t2568;
    let t7362 = 0.69345773920434148506e0 * t2139 * t7360;
    let t7363 = t7312 + 0.86682217400542685632e-1 * t7313 * t2187 + t7317 + 0.69861909304693186868e-1 * t5098 - 0.32927245914677557994e-1 * t5101 + 0.11643651550782197811e-1 * t5106 - 0.32927245914677557994e0 * t6293 * t7323 - 0.17336443480108537126e0 * t6583 * t7327 - 0.10401866088065122276e1 * t6106 * t7330 - 0.2600466522016280569e0 * t5108 * t7334 - 0.21951497276451705328e0 * t2122 * t7341 - 0.17336443480108537126e0 * t6132 * t7346 - 0.5200933044032561138e0 * t6139 * t7349 - 0.2600466522016280569e0 * t5108 * t7353 + 0.10975748638225852664e0 * t2122 * t7357 - t7362;
    (t7340, t7345, t7352, t7356, t7363)
}
