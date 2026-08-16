//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 915/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk915(t3640: f64, t6270: f64, t11947: f64, t6274: f64, t5385: f64, t604: f64, t1409: f64, t65: f64, t67: f64, t5392: f64, t9287: f64, t9300: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19267 = t6270 * t3640;
    let t19270 = t6274 * t11947;
    let t19299 = t5385 * t604;
    let t19322 = t1409 * t65 * t67;
    let t19368 = t9287 * t5392;
    let t19390 = t9300 * t5392;
    (t19267, t19270, t19299, t19322, t19368, t19390)
}
