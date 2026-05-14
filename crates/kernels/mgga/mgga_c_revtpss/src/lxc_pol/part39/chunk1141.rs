//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1141/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1141<F: Float>(t10503: F, t10507: F, t10511: F, t10984: F, t10987: F, t14998: F, t15004: F, t15006: F, t15010: F, t15011: F, t15015: F, t15018: F, t2829: F, t4474: F, t887: F, t4533: F, t886: F) -> (F, F) {
    let t15022 = -0.73171657588172351096e-2 * t14998 - t10503 - 0.23131639038696784278e-2 * t10507 + 0.2601984143835408805e-1 * t10511 - 0.11565819519348392139e-2 * t15004 + t10984 - 0.13009920719177044025e-1 * t15006 + t15010 - 0.13170898365871023197e1 * t15011 * t887 + 0.65049603595885220126e-3 * t15015 + 0.11565819519348392139e-2 * t15018 - t10987 - 0.65854491829355115987e0 * t4474 * t2829;
    let t15029 = t4533 * t886;
    (t15022, t15029)
}
