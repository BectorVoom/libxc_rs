//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 58/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk58(t12: f64, t13: f64, t138: f64, t145: f64) -> (f64, f64, f64) {
    let t176 = 0.51785e1_f64 * t13 + 0.905775e0_f64 * t12 + 0.1100325e0_f64 * t138 + 0.1241775e0_f64 * t145;
    let t179 = 1.0_f64 + 0.29608749977793437516e2_f64 / t176;
    let t180 = f64::ln(t179);
    (t176, t179, t180)
}
