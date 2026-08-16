//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1209/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1209(t1580: f64, t2440: f64, t2439: f64, t1569: f64, t2453: f64, t2458: f64, t10503: f64, t10507: f64, t10511: f64, t10984: f64, t10987: f64, t14998: f64, t15004: f64, t15006: f64, t15010: f64, t15011: f64, t2829: f64, t4474: f64, t887: f64) -> f64 {
    let t15014 = t2440 * t1580;
    let t15015 = t2439 * t15014;
    let t15017 = t2453 * t1569;
    let t15018 = t15017 * t2458;
    let t15022 = -0.73171657588172351096e-2_f64 * t14998 - t10503 - 0.23131639038696784278e-2_f64 * t10507 + 0.2601984143835408805e-1_f64 * t10511 - 0.11565819519348392139e-2_f64 * t15004 + t10984 - 0.13009920719177044025e-1_f64 * t15006 + t15010 - 0.13170898365871023197e1_f64 * t15011 * t887 + 0.65049603595885220126e-3_f64 * t15015 + 0.11565819519348392139e-2_f64 * t15018 - t10987 - 0.65854491829355115987e0_f64 * t4474 * t2829;
    t15022
}
