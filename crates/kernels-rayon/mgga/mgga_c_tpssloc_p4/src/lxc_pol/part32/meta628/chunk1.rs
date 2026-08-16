//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2039/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2039(t25324: f64, t6562: f64, t794: f64, t23030: f64, t25258: f64, t22893: f64, t23164: f64, t25306: f64, t7524: f64, t81612: f64, t81613: f64, t4250: f64, t81749: f64) -> (f64, f64, f64, f64, f64) {
    let t87153 = t6562 * t794 * t25324;
    let t87154 = 0.82246703342411321824e-2_f64 * t87153;
    let t87155 = t23030 * t25258;
    let t87165 = t23164 * t22893 * t25306;
    let t87166 = 0.16449340668482264365e-1_f64 * t87165;
    let t87177 = t81612 * t81613 * t7524;
    let t87197 = t81749 * t4250;
    (t87154, t87155, t87166, t87177, t87197)
}
