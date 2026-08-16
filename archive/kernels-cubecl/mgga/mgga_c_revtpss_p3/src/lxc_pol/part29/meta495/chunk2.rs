//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1803/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1803<F: Float>(t25921: F, t26232: F, t26235: F, t26238: F, t26251: F, t26253: F, t26263: F, t26266: F, t26268: F, t26272: F, t28781: F, t28783: F, t28792: F, t28796: F, t7295: F, t8100: F) -> F {
    let t28799 = -F::cast_from(0.72280234901709995518e-2_f64) * t26232 + F::cast_from(0.25702851531048074406e-1_f64) * t28781 - F::cast_from(0.14456046980341999104e-1_f64) * t28783 - F::cast_from(0.14456046980341999104e-1_f64) * t26235 - t26238 + t26251 + F::cast_from(0.9757440539382783019e-2_f64) * t26253 - t26263 - F::cast_from(0.9757440539382783019e-2_f64) * t26266 + F::cast_from(0.4336814094102599731e0_f64) * t25921 * t8100 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t28792 + F::cast_from(0.12851425765524037203e-1_f64) * t26268 - F::cast_from(0.12851425765524037203e-1_f64) * t28796 + F::cast_from(0.72280234901709995518e-2_f64) * t26272;
    t28799
}
