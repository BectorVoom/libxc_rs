//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 735/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk735(t107: f64, t1539: f64, t209: f64, t6247: f64, t837: f64, t874: f64, t235: f64, t325: f64, t6477: f64, t117: f64, t1915: f64, t875: f64, t899: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28317 = t1539 * t107;
    let t29439 = t6247 * t209;
    let t29837 = t837 * t874;
    let t29838 = t235 * t29837;
    let t30080 = t6477 * t325;
    let t30174 = t28317 * t117;
    let t30177 = t1915 * t107;
    let t30204 = t899 * t875;
    (t29439, t29837, t29838, t30080, t30174, t30177, t30204)
}
