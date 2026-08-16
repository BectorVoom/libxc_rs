//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2629/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2629(t1340: f64, t53909: f64, t16217: f64, t3866: f64, t1827: f64, t39947: f64, t16314: f64, t16398: f64, t16387: f64, t40138: f64, t5303: f64, t12283: f64, t16366: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54178 = t53909 * t1340;
    let t54191 = t3866 * t16217;
    let t54198 = t39947 * t1827;
    let t54202 = t16398 * t16314;
    let t54213 = t16398 * t16387;
    let t54220 = t40138 * t5303;
    let t54222 = t12283 * t16366;
    (t54178, t54191, t54198, t54202, t54213, t54220, t54222)
}
