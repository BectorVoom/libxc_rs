//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 981/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk981(t33409: f64, t6547: f64, t1888: f64, t31333: f64, t86873: f64, t1880: f64, t8547: f64, t87782: f64, t23204: f64, t33408: f64, t6562: f64, t33447: f64, t81651: f64, t82074: f64) -> (f64, f64, f64, f64, f64) {
    let t121296 = t6547 * t33409;
    let t121299 = t1888 * t86873 * t31333;
    let t121302 = t1880 * t87782 * t8547;
    let t121305 = t6562 * t23204 * t33408;
    let t121308 = t81651 * t82074 * t33447;
    (t121296, t121299, t121302, t121305, t121308)
}
