//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 617/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk617(t2186: f64, t2310: f64, t2289: f64, t2286: f64, t2283: f64, t1614: f64, t36: f64, t262: f64, t2103: f64, t1587: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8692 = t2186 * t2310;
    let t8694 = t2186 * t2289;
    let t8696 = t2186 * t2286;
    let t8698 = t2186 * t2283;
    let t8700 = t36 * t1614;
    let t8701 = t262 * t8700;
    let t8702 = t2103 * t8701;
    let t8704 = t36 * t1587;
    (t8692, t8694, t8696, t8698, t8700, t8701, t8702, t8704)
}
