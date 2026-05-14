//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1067/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1067<F: Float>(t11199: F, t3275: F, t9573: F, t42978: F, t42980: F, t42982: F, t42985: F, t42988: F, t42991: F, t42994: F, t42996: F, t42999: F, t43002: F, t43004: F, t39403: F, t41352: F, t41353: F, t41354: F, t41367: F, t41369: F, t41372: F, t43009: F, t43012: F, t43015: F, t43018: F, t43021: F) -> (F, F, F) {
    let t44168 = t3275 * t11199 * t9573 / 2.0;
    let t44181 = -0.46230515946956099003e0 * t42978 + 0.32927245914677557992e0 * t42980 + 0.21951497276451705328e0 * t42982 - 0.17336443480108537126e0 * t42985 - 0.52009330440325611378e0 * t42988 + 0.34672886960217074252e0 * t42991 - 0.10401866088065122276e1 * t42994 - 0.17336443480108537126e0 * t42996 + 0.86682217400542685632e-1 * t42999 + 0.2600466522016280569e0 * t43002 - 0.51220160311720645765e0 * t43004;
    let t44188 = -t41352 - t41353 - 0.21951497276451705328e0 * t43009 - t41354 - 0.17336443480108537126e0 * t43012 + 0.5200933044032561138e0 * t43015 + 0.17336443480108537126e0 * t43018 - 0.65854491829355115984e0 * t43021 + t41367 - t41369 - 0.92461031893912198008e0 * t39403 + t41372;
    (t44168, t44181, t44188)
}
