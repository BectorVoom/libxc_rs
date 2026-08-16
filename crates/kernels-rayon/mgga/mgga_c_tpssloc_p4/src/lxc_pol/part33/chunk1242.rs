//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1242/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1242(t10375: f64, t1942: f64, t1014: f64, t10469: f64, t363: f64, t3127: f64, t3200: f64, t83015: f64, t25511: f64, t6743: f64, t23592: f64, t23631: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83080 = t1942 * t10375 / 5184.0_f64;
    let t83142 = t10469 * t1014 * t363;
    let t83196 = t10469 * t3127 * t363;
    let t83215 = t3200 * t83015;
    let t83233 = t6743 * t25511;
    let t83239 = t23631 * t974 * t23592;
    (t83080, t83142, t83196, t83215, t83233, t83239)
}
