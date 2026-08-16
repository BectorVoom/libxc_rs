//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 785/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk785(t1715: f64, t3634: f64, t247: f64, t1261: f64, t1260: f64, t1785: f64) -> (f64, f64, f64) {
    let t5377 = t3634 * t1715;
    let t5378 = t247 * t5377;
    let t5379 = t1261 * t5378;
    let t5381 = t1785 * t1260;
    (t5378, t5379, t5381)
}
