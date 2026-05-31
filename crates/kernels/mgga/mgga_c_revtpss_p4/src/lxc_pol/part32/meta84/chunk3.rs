//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 517/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk517<F: Float>(t1749: F, t448: F, t1182: F, t1185: F, t1717: F, t1724: F, t1727: F, t1730: F) -> (F, F) {
    let t1750 = t1749 * t448;
    let t1756 = F::cast_from(0.258925e1_f64) * t1724 - t1182 + F::cast_from(0.301925e0_f64) * t1717 + F::cast_from(0.16504875e0_f64) * t1727 - t1185 + F::cast_from(0.82785e-1_f64) * t1730;
    (t1750, t1756)
}
