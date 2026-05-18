//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 979/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk979<F: Float>(t2067: F, t25391: F, t26541: F, t26545: F, t26557: F, t26558: F, t26561: F, t26564: F, t26578: F, t27199: F, t27275: F, t27353: F, t28426: F, t28434: F, t28436: F, t28439: F, t28442: F, t28449: F, t7415: F) -> F {
    let t28453 = -F::new(0.8673628188205199462e0) * t27353 * t28426 - F::new(0.14456046980341999104e-1) * t26541 + F::new(0.72280234901709995518e-2) * t26545 - t26557 - F::new(0.4336814094102599731e0) * t27275 * t2067 - F::new(0.12851425765524037203e-1) * t26558 - F::new(0.9757440539382783019e-2) * t28434 - F::new(0.8673628188205199462e0) * t25391 * t28436 + F::new(0.4336814094102599731e0) * t27353 * t28439 - F::new(0.8673628188205199462e0) * t25391 * t28442 + F::new(0.8673628188205199462e0) * t27199 * t7415 - F::new(0.54878743191129263322e-2) * t28449 + F::new(0.54878743191129263322e-2) * t26561 + F::new(0.9757440539382783019e-2) * t26564 + t26578;
    t28453
}
