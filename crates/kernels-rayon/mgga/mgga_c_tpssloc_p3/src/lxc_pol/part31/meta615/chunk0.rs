//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1862/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1862(t17635: f64, t605: f64, t19334: f64, t2235: f64, t5392: f64, t19534: f64, t88: f64, t1458: f64, t4025: f64, t5493: f64, t649: f64, t5464: f64, t81442: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96559 = t605 * t17635;
    let t96562 = t605 * t19334;
    let t96646 = t2235 * t5392;
    let t96657 = t88 * t19534;
    let t96683 = t4025 * t1458;
    let t96709 = t649 * t5493;
    let t96713 = t81442 * t5464;
    (t96559, t96562, t96646, t96657, t96683, t96709, t96713)
}
