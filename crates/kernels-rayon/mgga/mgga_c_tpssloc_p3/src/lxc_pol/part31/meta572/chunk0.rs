//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1806/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1806(t1887: f64, t81959: f64, t22690: f64, t23171: f64, t25319: f64, t23143: f64, t7525: f64, t25238: f64, t6579: f64, t22893: f64, t23164: f64, t25312: f64) -> (f64, f64, f64, f64, f64) {
    let t87642 = t81959 * t1887;
    let t87653 = t23171 * t22690 * t25319;
    let t87666 = t23143 * t7525;
    let t87668 = t6579 * t25238;
    let t87679 = t23164 * t22893 * t25312;
    (t87642, t87653, t87666, t87668, t87679)
}
