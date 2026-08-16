//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1087/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1087(t14142: f64, t4582: f64, t12648: f64, t4583: f64, t13559: f64, t977: f64, t2960: f64, t4603: f64, t1606: f64, t698: f64, t973: f64, t1043: f64, t2770: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14143 = t4582 * t14142;
    let t14146 = t4583 * t12648;
    let t14147 = t4582 * t14146;
    let t14152 = t977 * t13559;
    let t14158 = t2960 * t4603 / 162.0_f64;
    let t14159 = t698 * t1606;
    let t14160 = t973 * t14159;
    let t14164 = t1043 * t2770;
    (t14143, t14147, t14152, t14158, t14160, t14164)
}
