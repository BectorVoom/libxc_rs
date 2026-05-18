//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 971/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk971<F: Float>(t25921: F, t26232: F, t26235: F, t26238: F, t26251: F, t26253: F, t26263: F, t26266: F, t26268: F, t26272: F, t28781: F, t28783: F, t28792: F, t28796: F, t7295: F, t8100: F) -> F {
    let t28799 = -F::new(0.72280234901709995518e-2) * t26232 + F::new(0.25702851531048074406e-1) * t28781 - F::new(0.14456046980341999104e-1) * t28783 - F::new(0.14456046980341999104e-1) * t26235 - t26238 + t26251 + F::new(0.9757440539382783019e-2) * t26253 - t26263 - F::new(0.9757440539382783019e-2) * t26266 + F::new(0.4336814094102599731e0) * t25921 * t8100 + F::new(0.4336814094102599731e0) * t7295 * t28792 + F::new(0.12851425765524037203e-1) * t26268 - F::new(0.12851425765524037203e-1) * t28796 + F::new(0.72280234901709995518e-2) * t26272;
    t28799
}
