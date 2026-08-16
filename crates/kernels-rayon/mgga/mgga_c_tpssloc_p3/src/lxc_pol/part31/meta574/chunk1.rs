//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1810/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1810(t23030: f64, t25205: f64, t23164: f64, t7479: f64, t82133: f64, t23204: f64, t25216: f64, t6562: f64, t1519: f64, t212: f64, t23171: f64, t6554: f64) -> (f64, f64, f64, f64) {
    let t87898 = t23030 * t25205;
    let t87901 = t23164 * t82133 * t7479;
    let t87910 = t6562 * t23204 * t25216;
    let t87915 = t23171 * t212 * t1519 * t6554;
    (t87898, t87901, t87910, t87915)
}
