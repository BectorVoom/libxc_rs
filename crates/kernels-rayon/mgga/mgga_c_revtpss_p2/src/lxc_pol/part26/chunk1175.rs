//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1175/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1175(t93020: f64, t93022: f64, t93026: f64, t93028: f64, t93031: f64, t93035: f64, t93037: f64, t93039: f64, t93041: f64, t93043: f64, t93045: f64, t93049: f64, t93051: f64, t93055: f64) -> f64 {
    let t95684 = 0.28900264064772933812e-2_f64 * t93020;
    let t95698 = -t95684 - 0.20579528696673473747e-1_f64 * t93022 + 0.30492001685571196935e-3_f64 * t93026 + 0.12004725073059526352e-1_f64 * t93028 - 0.68598428988911579154e-3_f64 * t93031 + 0.16262400898971305032e-2_f64 * t93035 + 0.51448821741683684367e-1_f64 * t93037 + 0.51448821741683684367e-2_f64 * t93039 - 0.85748036236139473944e-3_f64 * t93041 - 0.15246000842785598468e-3_f64 * t93043 + 0.12004725073059526352e-1_f64 * t93045 - 0.68026775414003982662e-1_f64 * t93049 - 0.85748036236139473944e-3_f64 * t93051 - 0.24009450146119052704e-1_f64 * t93055;
    t95698
}
