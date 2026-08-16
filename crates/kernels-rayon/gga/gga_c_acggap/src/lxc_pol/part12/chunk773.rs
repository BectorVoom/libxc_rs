//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 773/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk773(t464: f64, t8331: f64, t633: f64, t864: f64, t2132: f64, t7885: f64, t157: f64, t2217: f64, t406: f64, t2152: f64, t862: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8332 = t8331 * t464;
    let t8336 = t633 * t864;
    let t8337 = t2132 * t8336;
    let t8339 = 0.26020884564615598386e1_f64 * t7885 * t8337;
    let t8341 = t2217 * t406 * t157;
    let t8342 = t2152 * t8341;
    let t8347 = t862 * t633;
    let t8349 = 0.13170898365871023197e1_f64 * t8347 * t865;
    (t8332, t8336, t8337, t8339, t8342, t8347, t8349)
}
