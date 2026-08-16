//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 780/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk780(t1389: f64, t2713: f64, t3964: f64, t1414: f64, t3889: f64, t828: f64, t2668: f64, t550: f64, t816: f64, t1379: f64, t1408: f64, t2482: f64, t27: f64) -> (f64, f64, f64, f64) {
    let t3967 = 0.45178982497454656791e-5_f64 * t3964 * t2713 * t1389;
    let t3970 = t1414 * t828 * t3889;
    let t3974 = t2668 * t550 * t816;
    let t3976 = 0.13552000749142754193e-3_f64 * t1379 * t3974;
    let t3978 = t2482 * t1408 * t27;
    (t3967, t3970, t3976, t3978)
}
