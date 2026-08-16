//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 727/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk727(t321: f64, t503: f64, t325: f64, t6477: f64, t622: f64, t794: f64, t117: f64, t28317: f64, t875: f64, t899: f64, t1540: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29933 = t503 * t321;
    let t30080 = t6477 * t325;
    let t30137 = t622 * t794;
    let t30174 = t28317 * t117;
    let t30204 = t899 * t875;
    let t30221 = t1540 * t117;
    let t30510 = t833 * t117;
    (t29933, t30080, t30137, t30174, t30204, t30221, t30510)
}
