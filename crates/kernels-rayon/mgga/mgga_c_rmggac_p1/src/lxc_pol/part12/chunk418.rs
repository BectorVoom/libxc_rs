//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 418/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk418(t151: f64, t4103: f64, t5: f64, t1034: f64, t421: f64, t155: f64, t1009: f64, t422: f64, t389: f64, t1012: f64, t1132: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4106 = 0.34450798614814814813e-2_f64 * t5 * t4103 * t151;
    let t4107 = t1034 * t421;
    let t4108 = t155 * t4107;
    let t4111 = 60.0_f64 * t1009 * t422;
    let t4114 = t1009 * t389;
    let t4116 = t1012 * t422;
    let t4118 = t1012 * t389;
    let t4120 = t381 * t1132;
    (t4106, t4108, t4111, t4114, t4116, t4118, t4120)
}
