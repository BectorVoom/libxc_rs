//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1111/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1111(t13953: f64, t14004: f64, t14050: f64, t14074: f64, t14120: f64, t14170: f64, t14233: f64, t14523: f64, t349: f64, t225: f64, t4658: f64, t1625: f64, t3020: f64) -> (f64, f64, f64, f64) {
    let t14526 = t13953 + t14004 + t14050 + t14074 + t14120 + t14170 + t14233 + t14523;
    let t14527 = t349 * t14526;
    let t14529 = t4658 * t225;
    let t14532 = t3020 * t1625;
    (t14526, t14527, t14529, t14532)
}
