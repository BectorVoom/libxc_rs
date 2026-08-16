//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1334/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1334(t1824: f64, t22705: f64, t22852: f64, t550: f64, t59: f64, t22827: f64, t26297: f64, t6943: f64, t26301: f64, t26322: f64, t6936: f64, t1831: f64, t31176: f64) -> (f64, f64, f64, f64, f64) {
    let t120363 = t22852 * t22705 * t59 * t1824 * t550;
    let t120366 = t22827 * t6943 * t26297;
    let t120369 = t22827 * t6943 * t26301;
    let t120372 = t6936 * t6943 * t26322;
    let t120375 = t31176 * t1831;
    (t120363, t120366, t120369, t120372, t120375)
}
