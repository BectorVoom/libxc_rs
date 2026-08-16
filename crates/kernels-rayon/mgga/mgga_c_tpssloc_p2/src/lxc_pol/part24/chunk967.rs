//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 967/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk967(t10431: f64, t10513: f64, t10929: f64, t11005: f64, t349: f64, t225: f64, t3167: f64, t3166: f64, t990: f64, t10358: f64, t381: f64, t1049: f64, t3020: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11007 = t10431 + t10513 + t10929 + t11005;
    let t11008 = t349 * t11007;
    let t11010 = t3167 * t225;
    let t11013 = t990 * t3166;
    let t11016 = t10358 * t381;
    let t11018 = t3020 * t1049;
    (t11007, t11008, t11010, t11013, t11016, t11018)
}
