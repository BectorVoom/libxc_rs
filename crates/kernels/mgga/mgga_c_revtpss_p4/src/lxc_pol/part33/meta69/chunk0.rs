//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 450/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk450<F: Float>(t1362: F, t1364: F, t535: F, t795: F, t159: F, t540: F, t216: F) -> (F, F, F, F) {
    let t1366 = F::cast_from(0.9757440539382783019e-2_f64) * t1362 * t1364;
    let t1368 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t795 * t535;
    let t1369 = t159 * t540;
    let t1370 = t216 * t1369;
    (t1366, t1368, t1369, t1370)
}
