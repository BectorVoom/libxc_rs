//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 319/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk319<F: Float>(t555: F, t72: F, t1432: F, t686: F, t1385: F, t565: F) -> (F, F, F, F) {
    let t1433 = t555 * t72;
    let t1436 = F::new(0.9757440539382783019e-2) * t1432 * t1433 * t686;
    let t1437 = t1385 * t555;
    let t1450 = F::new(1.0) / t565;
    (t1433, t1436, t1437, t1450)
}
