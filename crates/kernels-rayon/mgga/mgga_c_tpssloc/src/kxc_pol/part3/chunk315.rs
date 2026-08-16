//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 315/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk315(t1003: f64, t68: f64, t369: f64, t191: f64) -> (f64, f64, f64, f64) {
    let t1004 = t1003 * t68;
    let t1005 = t1004 * t369;
    let t1008 = t191 * t191;
    let t1009 = 1.0_f64 / t1008;
    (t1004, t1005, t1008, t1009)
}
