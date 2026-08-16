//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1823/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1823(t20217: f64, t31: f64, t65: f64, t1426: f64, t5399: f64, t1410: f64, t5427: f64, t1409: f64, t5392: f64) -> (f64, f64, f64, f64, f64) {
    let t20218 = t31 * t20217;
    let t20219 = t20218 * t65;
    let t20222 = t5399 * t1426;
    let t20227 = t1410 * t5427;
    let t20234 = t5392 * t1409;
    (t20218, t20219, t20222, t20227, t20234)
}
