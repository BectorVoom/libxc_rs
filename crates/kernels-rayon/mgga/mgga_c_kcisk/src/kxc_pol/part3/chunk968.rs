//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 968/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk968(t1455: f64, t4169: f64, t4171: f64, t4165: f64, t4321: f64, t1457: f64, t475: f64, t1520: f64, t4170: f64, t13369: f64, t6322: f64, t4230: f64) -> (f64, f64, f64, f64, f64) {
    let t14287 = t1455 * t4169;
    let t14289 = 6.0_f64 * t14287 * t4171;
    let t14291 = 3.0_f64 * t4165 * t4321;
    let t14292 = t1457 * t1457;
    let t14293 = 1.0_f64 / t14292;
    let t14294 = t475 * t14293;
    let t14295 = t4171 * t1520;
    let t14297 = 6.0_f64 * t14294 * t14295;
    let t14298 = t1520 * t4321;
    let t14300 = 6.0_f64 * t4170 * t14298;
    let t14301 = t6322 * t13369;
    let t14302 = t4230 * t14301;
    (t14289, t14291, t14297, t14300, t14302)
}
