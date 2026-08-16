//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1209/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1209<F: Float>(t107063: F, t107065: F, t107067: F, t107070: F, t107074: F, t107077: F, t107084: F, t107086: F, t107088: F, t107090: F, t91149: F, t91167: F, t97219: F, t97238: F, t97240: F, t97253: F, t97261: F, t97263: F, t97283: F) -> F {
    let t107802 = t107063 / F::cast_from(64.0_f64) + t107065 / F::cast_from(128.0_f64) + t107067 / F::cast_from(64.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t97219 + t107070 / F::cast_from(64.0_f64) - F::cast_from(0.10173934535723378495e0_f64) * t97238 + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t97240 - t107074 / F::cast_from(256.0_f64) + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t97253 + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t107077 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t97261 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t97263 - F::cast_from(119.0_f64) / F::cast_from(288.0_f64) * t91149 - F::cast_from(35.0_f64) / F::cast_from(96.0_f64) * t97283 - F::cast_from(0.67826230238155856633e-1_f64) * t91167 - t107084 / F::cast_from(768.0_f64) - t107086 / F::cast_from(256.0_f64) - t107088 / F::cast_from(256.0_f64) - t107090 / F::cast_from(128.0_f64);
    t107802
}
