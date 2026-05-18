//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1001/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1001<F: Float>(t14224: F, t26304: F, t28845: F, t7289: F, t26356: F, t26361: F, t26363: F, t27868: F, t28826: F, t28830: F, t28838: F, t28841: F, t28846: F, t28850: F, t28853: F, t7292: F, t7295: F, t7532: F, t7917: F, t8104: F) -> F {
    let t28855 = t26304 * t14224;
    let t28858 = t7289 * t28845;
    let t28861 = -F::new(0.54878743191129263322e-2) * t28826 + F::new(0.4336814094102599731e0) * t7295 * t28830 - F::new(0.4336814094102599731e0) * t7917 * t7532 - F::new(0.4336814094102599731e0) * t7292 * t8104 + F::new(0.9757440539382783019e-2) * t28838 + F::new(0.8673628188205199462e0) * t7295 * t28841 + F::new(0.72280234901709995518e-2) * t28846 + F::new(0.4336814094102599731e0) * t7295 * t28850 - F::new(0.9757440539382783019e-2) * t28853 + F::new(0.4336814094102599731e0) * t27868 * t28855 - F::new(0.12851425765524037203e-1) * t28858 - F::new(0.54878743191129263322e-2) * t26356 - t26361 + t26363;
    t28861
}
