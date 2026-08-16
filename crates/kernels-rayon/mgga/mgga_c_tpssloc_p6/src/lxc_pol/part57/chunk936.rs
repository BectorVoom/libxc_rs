//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 936/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk936(t12571: f64, t31680: f64, t115876: f64, t33564: f64, t31688: f64, t33572: f64, t45844: f64, t8511: f64, t33115: f64, t31687: f64, t8515: f64, t33409: f64, t6547: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121058 = t12571 * t31680;
    let t121064 = t115876 * t33564;
    let t121066 = t31688 * t33572;
    let t121094 = t45844 * t8511;
    let t121121 = t31688 * t33115;
    let t121124 = t12571 * t31687 * t8515;
    let t121296 = t6547 * t33409;
    (t121058, t121064, t121066, t121094, t121121, t121124, t121296)
}
