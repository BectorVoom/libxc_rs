//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1209/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1209<F: Float>(t1580: F, t2440: F, t2439: F, t1569: F, t2453: F, t2458: F, t10503: F, t10507: F, t10511: F, t10984: F, t10987: F, t14998: F, t15004: F, t15006: F, t15010: F, t15011: F, t2829: F, t4474: F, t887: F) -> F {
    let t15014 = t2440 * t1580;
    let t15015 = t2439 * t15014;
    let t15017 = t2453 * t1569;
    let t15018 = t15017 * t2458;
    let t15022 = -F::cast_from(0.73171657588172351096e-2_f64) * t14998 - t10503 - F::cast_from(0.23131639038696784278e-2_f64) * t10507 + F::cast_from(0.2601984143835408805e-1_f64) * t10511 - F::cast_from(0.11565819519348392139e-2_f64) * t15004 + t10984 - F::cast_from(0.13009920719177044025e-1_f64) * t15006 + t15010 - F::cast_from(0.13170898365871023197e1_f64) * t15011 * t887 + F::cast_from(0.65049603595885220126e-3_f64) * t15015 + F::cast_from(0.11565819519348392139e-2_f64) * t15018 - t10987 - F::cast_from(0.65854491829355115987e0_f64) * t4474 * t2829;
    t15022
}
