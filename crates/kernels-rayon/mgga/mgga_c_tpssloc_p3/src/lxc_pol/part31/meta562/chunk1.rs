//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1792/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1792(t252: f64, t4119: f64, t22690: f64, t7520: f64, t81573: f64, t25324: f64, t6562: f64, t794: f64, t23030: f64, t25258: f64, t22893: f64, t23164: f64, t25306: f64) -> (f64, f64, f64, f64, f64) {
    let t87130 = t252 * t4119;
    let t87140 = t81573 * t22690 * t7520;
    let t87153 = t6562 * t794 * t25324;
    let t87155 = t23030 * t25258;
    let t87165 = t23164 * t22893 * t25306;
    (t87130, t87140, t87153, t87155, t87165)
}
