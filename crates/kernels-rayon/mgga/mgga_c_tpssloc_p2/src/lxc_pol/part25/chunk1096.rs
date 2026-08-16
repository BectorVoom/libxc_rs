//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1096/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1096(t2230: f64, t22843: f64, t213: f64, t22847: f64, t22842: f64, t531: f64, t598: f64, t12156: f64, t1998: f64, t236: f64, t12328: f64, t2003: f64) -> (f64, f64, f64) {
    let t80887 = t2230 * t22843;
    let t80888 = t80887 * t213;
    let t80889 = t80888 * t22847;
    let t80893 = t598 / t22842 / t531;
    let t80894 = t80893 * t213;
    let t80897 = t80894 * t1998 * t236 * t12156;
    let t80899 = t2003 * t12328;
    (t80889, t80897, t80899)
}
