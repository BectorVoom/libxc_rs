//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 317/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk317<F: Float>(t555: F, t72: F, t1432: F, t686: F, t1385: F, t565: F) -> (F, F, F, F) {
    let t1433 = t555 * t72;
    let t1436 = F::cast_from(0.9757440539382783019e-2_f64) * t1432 * t1433 * t686;
    let t1437 = t1385 * t555;
    let t1450 = F::cast_from(1.0_f64) / t565;
    (t1433, t1436, t1437, t1450)
}
