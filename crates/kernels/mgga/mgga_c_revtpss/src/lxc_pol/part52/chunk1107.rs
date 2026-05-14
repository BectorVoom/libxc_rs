//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1107/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1107<F: Float>(t128493: F, t128495: F, t128497: F, t128499: F, t128510: F, t128513: F, t128517: F, t128519: F, t128521: F, t25805: F, t28025: F, t28711: F, t28760: F, t33602: F, t6985: F, t7374: F, t7978: F) -> (F,) {
    let t128522 = -2.0 * t25805 * t7978 - 2.0 * t28025 * t7978 - 2.0 * t28711 * t6985 - 2.0 * t28760 * t6985 - 2.0 * t33602 * t7374 - t128493 - t128495 - t128497 - t128499 - t128510 - t128513 - t128517 - t128519 - t128521;
    (t128522,)
}
