//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2267/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2267(t15402: f64, t18225: f64, t3447: f64, t11589: f64, t18427: f64, t18221: f64, t15376: f64, t15399: f64, t15403: f64, t18409: f64, t15339: f64, t15419: f64, t18232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t64696 = t3447 * t15402 * t18225;
    let t64699 = t3447 * t11589 * t18427;
    let t64702 = t3447 * t15402 * t18221;
    let t64711 = t15376 * t15399;
    let t64713 = t15376 * t15403;
    let t64718 = t3447 * t11589 * t18409;
    let t64730 = t15376 * t15339;
    let t64733 = t3447 * t15419 * t18232;
    (t64696, t64699, t64702, t64711, t64713, t64718, t64730, t64733)
}
