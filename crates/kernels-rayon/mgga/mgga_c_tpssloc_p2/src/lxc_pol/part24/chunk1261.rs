//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1261/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1261(t213: f64, t80887: f64, t22847: f64, t22842: f64, t531: f64, t598: f64, t12156: f64, t1998: f64, t236: f64, t12328: f64, t2003: f64, t12248: f64, t59: f64) -> (f64, f64, f64, f64) {
    let t80888 = t80887 * t213;
    let t80889 = t80888 * t22847;
    let t80893 = t598 / t22842 / t531;
    let t80894 = t80893 * t213;
    let t80897 = t80894 * t1998 * t236 * t12156;
    let t80899 = t2003 * t12328;
    let t80900 = 595.0_f64 / 5184.0_f64 * t80899;
    let t80901 = t12248 * t59;
    (t80889, t80897, t80900, t80901)
}
