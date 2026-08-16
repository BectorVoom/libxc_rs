//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 950/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk950(t114790: f64, t23164: f64, t7479: f64, t23168: f64, t33419: f64, t33395: f64, t814: f64, t1484: f64, t2047: f64, t22893: f64, t33375: f64, t33383: f64, t6562: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121464 = t23164 * t114790 * t7479;
    let t121469 = t23168 * t33419;
    let t121488 = t814 * t33395;
    let t121495 = t2047 * t1484;
    let t121501 = t23164 * t22893 * t33375;
    let t121504 = t6562 * t794 * t33383;
    (t121464, t121469, t121488, t121495, t121501, t121504)
}
