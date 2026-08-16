//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 334/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk334(t20: f64, t982: f64, t414: f64, t24: f64, t287: f64, t209: f64, t421: f64, t416: f64, t415: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1241 = t982 * t20;
    let t1242 = t414 * t1241;
    let t1245 = t24 * t287;
    let t1247 = t209 * t1245 * t421;
    let t1249 = t416 * t1247 / 576.0_f64;
    let t1250 = t415 * t68;
    let t1251 = t414 * t1250;
    (t1241, t1242, t1245, t1247, t1249, t1251)
}
