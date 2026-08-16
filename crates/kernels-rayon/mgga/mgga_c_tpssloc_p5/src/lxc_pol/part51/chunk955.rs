//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 955/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk955(t1058: f64, t23418: f64, t210: f64, t6679: f64, t3139: f64, t6717: f64, t3113: f64, t6754: f64, t3107: f64, t6753: f64, t1012: f64, t1933: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23419 = t1058 * t23418;
    let t23422 = t6679 * t210;
    let t23425 = t6717 * t3139;
    let t23433 = t3113 * t6754;
    let t23436 = t6753 * t3107;
    let t23437 = t1012 * t23436;
    let t23442 = t1933 * t607;
    (t23419, t23422, t23425, t23433, t23437, t23442)
}
