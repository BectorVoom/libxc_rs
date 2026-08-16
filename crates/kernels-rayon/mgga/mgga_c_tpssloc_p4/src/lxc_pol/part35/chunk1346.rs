//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1346/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1346(t27817: f64, t7999: f64, t24574: f64, t29813: f64, t225: f64, t29665: f64, t8006: f64, t94490: f64, t29827: f64, t3640: f64, t2109: f64, t96461: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104589 = t7999 * t27817;
    let t104609 = t24574 * t29813;
    let t104635 = t29665 * t225;
    let t104647 = t94490 * t8006;
    let t104677 = t29827 * t3640;
    let t104735 = t2109 * t96461;
    (t104589, t104609, t104635, t104647, t104677, t104735)
}
