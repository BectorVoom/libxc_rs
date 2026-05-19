//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 862/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk862<F: Float>(t38203: F, t38204: F, t38205: F, t38206: F, t38210: F, t38211: F, t9268: F, t9269: F, t9270: F, t9271: F, t9741: F, t34544: F, t34545: F, t34548: F, t34551: F, t34554: F, t7304: F, t7308: F, t7319: F, t7340: F, t8467: F, t8470: F) -> (F, F) {
    let t44518 = t38203 - t38204 - t38205 + t38206 - t38210 - t38211 + t9268 - t9269 + t9270 - t9271 - t9741;
    let t44526 = t34544 - t34545 - t7304 - t7308 + t34548 + F::cast_from(0.14408463291498358381e-2_f64) * t8467 - F::cast_from(0.20496175532535769484e-3_f64) * t8470 - t7319 + t34551 - t34554 - t7340;
    (t44518, t44526)
}
