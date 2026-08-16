//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 581/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk581(t6744: f64, t6746: f64, t1004: f64, t1941: f64, t1014: f64, t1018: f64, t1012: f64, t1030: f64, t1940: f64, t354: f64, t1036: f64, t1942: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6747 = t6744 * t6746;
    let t6750 = t1004 * t1941;
    let t6753 = t1014 * sigma0;
    let t6754 = t6753 * t1018;
    let t6755 = t1012 * t6754;
    let t6758 = t1940 * t1030;
    let t6759 = t354 * t6758;
    let t6763 = t1942 * t1036 / 2304.0_f64;
    (t6747, t6750, t6753, t6754, t6755, t6758, t6759, t6763)
}
