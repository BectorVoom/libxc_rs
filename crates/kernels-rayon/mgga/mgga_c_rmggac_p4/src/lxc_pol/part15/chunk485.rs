//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 485/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk485(t221: f64, t446: f64, t6108: f64, t1468: f64, t1494: f64, t1875: f64, t4559: f64, t489: f64, t490: f64, t6067: f64, t1228: f64, t1900: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6110 = t221 * t6108 * t446;
    let t6113 = t1468 * t1494;
    let t6114 = t221 * t6113;
    let t6117 = t4559 * t1875;
    let t6120 = t489 * t490 * t6067;
    let t6123 = t1228 * t1900;
    (t6110, t6113, t6114, t6117, t6120, t6123)
}
