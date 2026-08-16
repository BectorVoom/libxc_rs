//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1345/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1345(t24574: f64, t29551: f64, t8003: f64, t94490: f64, t29694: f64, t1170: f64, t2121: f64, t29670: f64, t29678: f64, t7280: f64, t225: f64, t29687: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104504 = t24574 * t29551;
    let t104506 = t94490 * t8003;
    let t104509 = t24574 * t29694;
    let t104521 = t2121 * t1170 * t29670;
    let t104527 = t29678 * t7280;
    let t104556 = t29687 * t225;
    (t104504, t104506, t104509, t104521, t104527, t104556)
}
