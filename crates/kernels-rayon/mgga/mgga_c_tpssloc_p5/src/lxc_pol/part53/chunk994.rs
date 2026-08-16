//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 994/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk994(t1880: f64, t25216: f64, t31366: f64, t121401: f64, t6572: f64, t1888: f64, t23270: f64, t26729: f64, t33428: f64, t6562: f64, t794: f64, t114790: f64, t7488: f64) -> (f64, f64, f64, f64, f64) {
    let t121713 = t1880 * t31366 * t25216;
    let t121716 = t1880 * t121401 * t6572;
    let t121745 = t1888 * t23270 * t26729;
    let t121749 = t6562 * t794 * t33428;
    let t121753 = t6562 * t114790 * t7488;
    (t121713, t121716, t121745, t121749, t121753)
}
