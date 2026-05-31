//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1249/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1249<F: Float>(t128353: F, t2056: F, t128355: F, t34258: F, t7367: F, t128493: F, t128495: F, t128497: F, t128499: F, t128510: F, t128513: F, t25805: F, t28025: F, t28711: F, t28760: F, t33602: F, t6985: F, t7374: F, t7978: F) -> F {
    let t128517 = F::cast_from(2.0_f64) * t128353 * t2056;
    let t128519 = F::cast_from(2.0_f64) * t128355 * t2056;
    let t128521 = F::cast_from(2.0_f64) * t34258 * t7367;
    let t128522 = -F::cast_from(2.0_f64) * t25805 * t7978 - F::cast_from(2.0_f64) * t28025 * t7978 - F::cast_from(2.0_f64) * t28711 * t6985 - F::cast_from(2.0_f64) * t28760 * t6985 - F::cast_from(2.0_f64) * t33602 * t7374 - t128493 - t128495 - t128497 - t128499 - t128510 - t128513 - t128517 - t128519 - t128521;
    t128522
}
