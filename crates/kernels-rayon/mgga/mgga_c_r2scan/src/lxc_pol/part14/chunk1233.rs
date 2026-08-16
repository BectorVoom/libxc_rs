//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1233/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1233(t40232: f64, t40234: f64, t40241: f64, t40243: f64, t38147: f64, t38150: f64, t38153: f64, t38156: f64, t38158: f64, t38161: f64, t38649: f64, t38657: f64) -> f64 {
    let t41756 = 0.39029762157531132074e-1_f64 * t40232;
    let t41757 = 0.11708928647259339622e0_f64 * t40234;
    let t41762 = 0.93149212406257582492e-1_f64 * t40241;
    let t41763 = 0.39029762157531132074e-1_f64 * t40243;
    let t41766 = t41756 + t41757 - t38649 + 0.93149212406257582492e-1_f64 * t38147 + 0.32524801797942610063e-3_f64 * t38150 - 0.11565819519348392138e-2_f64 * t38153 + 0.27944763721877274748e0_f64 * t38156 - t41762 - t41763 + 0.12805040077930161442e0_f64 * t38158 - 0.93149212406257582492e-1_f64 * t38161 + t38657;
    t41766
}
