//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 585/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk585(t3071: f64, t4574: f64, t1023: f64, t1539: f64, t247: f64, t375: f64) -> (f64, f64, f64) {
    let t4575 = t3071 * t4574;
    let t4578 = t1539 * t1023;
    let t4579 = t3071 * t4578;
    let t4582 = t247 * t375;
    (t4575, t4579, t4582)
}
