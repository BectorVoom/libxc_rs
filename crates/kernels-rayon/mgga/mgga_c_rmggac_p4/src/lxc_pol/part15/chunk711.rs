//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 711/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk711(t10113: f64, t1953: f64, t668: f64, t72: f64, t2073: f64, t9877: f64, t1756: f64, t36: f64, t2079: f64, t262: f64, t570: f64, t8975: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10114 = 0.2993560425465952141e-1_f64 * t10113;
    let t10115 = t1953 * t668;
    let t10116 = t72 * t10115;
    let t10120 = t2073 * t9877;
    let t10122 = t36 * t1756;
    let t10124 = t2079 * t262 * t10122;
    let t10130 = t8975 * t570;
    (t10114, t10115, t10116, t10120, t10122, t10124, t10130)
}
