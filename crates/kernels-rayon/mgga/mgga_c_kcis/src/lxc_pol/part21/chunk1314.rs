//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1314/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1314(t7703: f64, t95890: f64, t1020: f64, t8047: f64, t92742: f64, t1749: f64, t303: f64, t3220: f64, t1768: f64, t92999: f64, t1092: f64, t1133: f64, t26760: f64, t43053: f64) -> (f64, f64, f64, f64, f64) {
    let t96173 = t7703 * t95890;
    let t96178 = t1020 * t92742 * t8047;
    let t96181 = t303 * t1749 * t3220;
    let t96184 = t303 * t92999 * t1768;
    let t96190 = t1092 * t26760 * t43053 * t1133;
    (t96173, t96178, t96181, t96184, t96190)
}
