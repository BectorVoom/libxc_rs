//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 239/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk239(t1156: f64, t197: f64, t461: f64, t495: f64, t225: f64, t226: f64) -> (f64, f64, f64, f64) {
    let t1157 = t197 * t1156;
    let t1168 = t461 * t495;
    let t1171 = t225 * t225;
    let t1173 = 1.0_f64 / t226 / t1171;
    (t1157, t1168, t1171, t1173)
}
