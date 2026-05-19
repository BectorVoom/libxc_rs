//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 822/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk822<F: Float>(t2139: F, t7360: F, t2122: F, t2187: F, t5098: F, t5101: F, t5106: F, t5108: F, t6106: F, t6132: F, t6139: F, t6293: F, t6583: F, t7312: F, t7313: F, t7317: F, t7323: F, t7327: F, t7330: F, t7334: F, t7341: F, t7346: F, t7349: F, t7353: F, t7357: F) -> F {
    let t7362 = F::cast_from(0.69345773920434148506e0_f64) * t2139 * t7360;
    let t7363 = t7312 + F::cast_from(0.86682217400542685632e-1_f64) * t7313 * t2187 + t7317 + F::cast_from(0.69861909304693186868e-1_f64) * t5098 - F::cast_from(0.32927245914677557994e-1_f64) * t5101 + F::cast_from(0.11643651550782197811e-1_f64) * t5106 - F::cast_from(0.32927245914677557994e0_f64) * t6293 * t7323 - F::cast_from(0.17336443480108537126e0_f64) * t6583 * t7327 - F::cast_from(0.10401866088065122276e1_f64) * t6106 * t7330 - F::cast_from(0.2600466522016280569e0_f64) * t5108 * t7334 - F::cast_from(0.21951497276451705328e0_f64) * t2122 * t7341 - F::cast_from(0.17336443480108537126e0_f64) * t6132 * t7346 - F::cast_from(0.5200933044032561138e0_f64) * t6139 * t7349 - F::cast_from(0.2600466522016280569e0_f64) * t5108 * t7353 + F::cast_from(0.10975748638225852664e0_f64) * t2122 * t7357 - t7362;
    t7363
}
