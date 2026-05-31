//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1290/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1290<F: Float>(t128869: F, t128871: F, t128874: F, t128876: F, t128878: F, t128880: F, t128882: F, t128891: F, t27060: F, t28704: F, t28711: F, t28727: F, t29432: F, t7586: F, t7984: F, t8764: F) -> F {
    let t131018 = -F::cast_from(2.0_f64) * t27060 * t7984 - F::cast_from(2.0_f64) * t28704 * t7586 - F::cast_from(2.0_f64) * t28711 * t7586 - t28727 * t8764 - F::cast_from(2.0_f64) * t29432 * t7984 + t128869 - t128871 - t128874 - t128876 - t128878 - t128880 - t128882 - t128891;
    t131018
}
