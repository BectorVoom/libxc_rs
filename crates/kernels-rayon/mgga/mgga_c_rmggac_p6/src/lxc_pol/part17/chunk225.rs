//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 225/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk225(t417: f64, t977: f64, t978: f64, t431: f64, t58: f64, t63: f64, t230: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t980 = t977 * t978 * t417;
    let t982 = 0.11696447245269292414e1_f64 * t431 * t980;
    let t983 = 1.0_f64 / t58;
    let t990 = 1.0_f64 / t63;
    let t1003 = t230 * t230;
    let t1004 = 1.0_f64 / t1003;
    (t980, t982, t983, t990, t1003, t1004)
}
