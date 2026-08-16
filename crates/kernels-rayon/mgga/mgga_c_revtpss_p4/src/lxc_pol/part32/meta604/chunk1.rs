//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1942/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1942(t18352: f64, t1945: f64, t807: f64, t61639: f64, t99062: f64, t27221: f64, t61725: f64, t6017: f64, t886: f64, t1955: f64, t27212: f64, t6022: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t106102 = t807 * t1945 * t18352;
    let t106104 = t99062 * t61639;
    let t106106 = t27221 * t61725;
    let t106143 = t6017 * t886;
    let t106172 = t1955 * t27212;
    let t106228 = t6022 * t886;
    (t106102, t106104, t106106, t106143, t106172, t106228)
}
