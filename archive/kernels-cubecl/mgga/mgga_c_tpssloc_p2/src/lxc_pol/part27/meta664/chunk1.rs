//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2331/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2331<F: Float>(t80767: F, t80776: F, t80761: F, t80769: F, t91183: F, t91185: F, t91187: F, t91189: F, t91192: F, t91196: F, t91200: F, t91204: F, t91206: F, t91210: F, t91212: F, t91215: F, t91216: F, t91218: F) -> F {
    let t91221 = F::cast_from(0.13565246047631171327e0_f64) * t80767;
    let t91223 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t80776;
    let t91224 = -F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t91183 - t91185 / F::cast_from(1536.0_f64) - t91187 / F::cast_from(768.0_f64) - t91189 / F::cast_from(1536.0_f64) - t91192 / F::cast_from(192.0_f64) - t91196 / F::cast_from(4.0_f64) - F::cast_from(0.67826230238155856634e-1_f64) * t91200 + F::cast_from(0.24223653656484234512e-2_f64) * t91204 - F::cast_from(0.31625325607076639502e-2_f64) * t91206 - F::cast_from(0.16956557559538964158e-1_f64) * t91210 - t91212 / F::cast_from(192.0_f64) - t91215 - t91216 / F::cast_from(1536.0_f64) - t91218 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t80761 - t91221 + F::cast_from(0.84782787797694820794e-2_f64) * t80769 - t91223;
    t91224
}
