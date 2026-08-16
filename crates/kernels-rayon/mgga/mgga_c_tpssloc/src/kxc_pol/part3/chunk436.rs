//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 436/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk436(t1395: f64, t3: f64, t576: f64, t112: f64) -> (f64, f64, f64) {
    let t1396 = t3 * t1395;
    let t1398 = t3 * t576;
    let t1401 = t576 * t112;
    (t1396, t1398, t1401)
}
