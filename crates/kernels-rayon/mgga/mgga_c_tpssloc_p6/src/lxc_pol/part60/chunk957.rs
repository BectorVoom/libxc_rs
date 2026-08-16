//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 957/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk957(t22892: f64, t22893: f64, t33276: f64, t22751: f64, t33277: f64, t552: f64, t7918: f64, t33245: f64, t6897: f64, t794: f64, t131: f64, t2240: f64, t27331: f64) -> (f64, f64, f64, f64, f64) {
    let t122533 = t22892 * t22893 * t33276;
    let t122535 = t22751 * t33277;
    let t122537 = t552 * t7918;
    let t122551 = t6897 * t794 * t33245;
    let t122945 = t2240 * t27331 * t131;
    (t122533, t122535, t122537, t122551, t122945)
}
