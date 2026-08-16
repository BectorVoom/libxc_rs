//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 903/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk903(t7244: f64, t8437: f64, t7255: f64, t9159: f64, t1614: f64, t1970: f64, t1971: f64, t209: f64, t476: f64, t511: f64, t30900: f64, t35972: f64, t739: f64) -> (f64, f64, f64, f64) {
    let t39977 = t7244 * t8437;
    let t39979 = t7255 * t9159;
    let t39985 = t1970 * t1971 * t511 * t1614 * t476 * t209;
    let t39994 = t739 * t35972 * t30900;
    (t39977, t39979, t39985, t39994)
}
