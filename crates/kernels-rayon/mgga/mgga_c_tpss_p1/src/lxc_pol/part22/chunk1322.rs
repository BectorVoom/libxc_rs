//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1322/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1322(t1006: f64, t3724: f64, t8096: f64, t19818: f64, t20047: f64, t44474: f64, t18246: f64, t64296: f64, t14076: f64, t61703: f64, t1497: f64, t2436: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64954 = t1006 * t3724;
    let t64957 = t8096 * t1006;
    let t64958 = t64957 * t19818;
    let t64966 = t20047 * t44474;
    let t64969 = t18246 * t64296;
    let t64972 = t61703 * t14076;
    let t64975 = t2436 * t1497;
    (t64954, t64958, t64966, t64969, t64972, t64975)
}
