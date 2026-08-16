//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1009/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1009(t1652: f64, t1970: f64, t1971: f64, t209: f64, t476: f64, t515: f64, t7244: f64, t8432: f64, t1475: f64, t839: f64, t880: f64, t236: f64, t794: f64, t9188: f64) -> (f64, f64, f64, f64) {
    let t42099 = t1970 * t1971 * t515 * t1652 * t476 * t209;
    let t42101 = t7244 * t8432;
    let t42109 = t1970 * t1971 * t880 * t1475 * t839;
    let t42114 = t1970 * t9188 * t236 * t1475 * t794;
    (t42099, t42101, t42109, t42114)
}
