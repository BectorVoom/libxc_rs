//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2020/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2020<F: Float>(t93644: F, t93645: F, t93646: F, t97236: F, t97238: F, t97240: F, t97242: F, t97244: F, t97247: F, t97249: F, t97251: F, t97253: F, t97255: F, t97257: F, t97259: F, t97261: F, t97263: F, t97266: F) -> F {
    let t102663 = F::cast_from(0.16149102437656156341e-2_f64) * t97236 - F::cast_from(0.33913115119077928317e-1_f64) * t97238 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t97240 - t97242 / F::cast_from(768.0_f64) - t97244 / F::cast_from(768.0_f64) - t97247 / F::cast_from(768.0_f64) - t97249 / F::cast_from(384.0_f64) - t97251 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t97253 - F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t97255 + F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t97257 - t97259 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t97261 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t97263 - t97266 / F::cast_from(192.0_f64) + t93644 + t93645 - t93646;
    t102663
}
