//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 937/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk937(t23204: f64, t33408: f64, t6562: f64, t33447: f64, t81651: f64, t82074: f64, t2717: f64, t7841: f64, t33448: f64, t81591: f64, t8547: f64, t86893: f64) -> (f64, f64, f64, f64, f64) {
    let t121305 = t6562 * t23204 * t33408;
    let t121308 = t81651 * t82074 * t33447;
    let t121349 = t2717 * t7841;
    let t121371 = t81591 * t33448;
    let t121399 = t6562 * t86893 * t8547;
    (t121305, t121308, t121349, t121371, t121399)
}
