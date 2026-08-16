//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 975/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk975(t22827: f64, t26297: f64, t6943: f64, t26301: f64, t26322: f64, t6936: f64, t1831: f64, t31176: f64, t1369: f64, t32717: f64, t31165: f64, t5314: f64, t8466: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120366 = t22827 * t6943 * t26297;
    let t120369 = t22827 * t6943 * t26301;
    let t120372 = t6936 * t6943 * t26322;
    let t120375 = t31176 * t1831;
    let t120377 = t32717 * t1369;
    let t120379 = t31165 * t1831;
    let t120381 = t8466 * t5314;
    (t120366, t120369, t120372, t120375, t120377, t120379, t120381)
}
