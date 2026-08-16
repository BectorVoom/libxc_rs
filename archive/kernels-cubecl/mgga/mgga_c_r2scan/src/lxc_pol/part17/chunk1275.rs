//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1275/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1275<F: Float>(t37431: F, t37438: F, t39069: F, t40319: F, t40331: F, t40334: F, t42196: F, t42197: F, t44161: F, t44165: F, t44168: F, t44519: F, t44524: F, t44526: F, t44530: F) -> F {
    let t44986 = -F::cast_from(0.20496175532535769483e-3_f64) * t40319 + t44161 - F::cast_from(0.1440846329149835838e-2_f64) * t37431 + F::cast_from(0.20496175532535769483e-3_f64) * t37438 + F::cast_from(0.325201597776800302e-2_f64) * t40331 - F::cast_from(0.78064147182743091554e-3_f64) * t40334 - t44165 + t39069 - t44168 + t42196 - t42197 - t44519 + t44524 + t44526 - t44530;
    t44986
}
