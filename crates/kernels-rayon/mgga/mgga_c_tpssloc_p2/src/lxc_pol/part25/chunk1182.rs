//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1182/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1182(t22519: f64, t7032: f64, t22537: f64, t23998: f64, t6495: f64, t39049: f64, t7025: f64, t39063: f64, t23966: f64, t9239: f64, t22546: f64, t22493: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t84203 = t22519 * t7032;
    let t84205 = t22537 * t7032;
    let t84207 = t6495 * t23998;
    let t84209 = t39049 * t7025;
    let t84216 = t39063 * t7025;
    let t84219 = t9239 * t23966;
    let t84220 = t84219 * t22546;
    let t84222 = t22493 * t7032;
    (t84203, t84205, t84207, t84209, t84216, t84220, t84222)
}
