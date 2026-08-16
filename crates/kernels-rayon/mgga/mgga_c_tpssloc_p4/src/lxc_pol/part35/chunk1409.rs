//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1409/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1409(t16311: f64, t3788: f64, t6414: f64, t6936: f64, t1339: f64, t20554: f64, t20563: f64, t221: f64, t26284: f64, t20442: f64, t22833: f64, t2002: f64, t20595: f64, t559: f64) -> (f64, f64, f64, f64, f64) {
    let t107183 = t6936 * t3788 * t16311 * t6414;
    let t107186 = t6936 * t1339 * t20554;
    let t107189 = t26284 * t221 * t20563;
    let t107198 = t22833 * t20442;
    let t107205 = t20595 * t2002 * t559;
    (t107183, t107186, t107189, t107198, t107205)
}
