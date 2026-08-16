//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 226/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk226(t33: f64, t77: f64, t74: f64, t184: f64, t154: f64, t73: f64, t294: f64, t384: f64, t381: f64, t422: f64, t389: f64, t195: f64, t498: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1007 = t77 * t33;
    let t1008 = 1.0_f64 / t1007;
    let t1009 = t74 * t1008;
    let t1011 = 20.0_f64 * t1009 * t184;
    let t1012 = t73 * t154;
    let t1014 = 12.0_f64 * t1012 * t184;
    let t1015 = t294 * t384;
    let t1017 = 32.0_f64 * t1015 * t184;
    let t1019 = 8.0_f64 * t381 * t422;
    let t1020 = t381 * t389;
    let t1022 = t195 * t498;
    (t1007, t1008, t1009, t1011, t1012, t1014, t1015, t1017, t1019, t1020, t1022)
}
