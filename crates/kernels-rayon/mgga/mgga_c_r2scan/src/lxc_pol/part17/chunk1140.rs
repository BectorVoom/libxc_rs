//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1140/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1140(t40175: f64, t40177: f64, t40180: f64, t40201: f64, t40215: f64, t40217: f64, t40222: f64, t40232: f64, t40234: f64, t40241: f64, t40243: f64, t40257: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41734 = 0.46230515946956099004e0_f64 * t40175;
    let t41735 = 0.13869154784086829701e1_f64 * t40177;
    let t41736 = 0.13869154784086829701e1_f64 * t40180;
    let t41743 = 0.19043987679069580389e-1_f64 * t40201;
    let t41748 = 0.19514881078765566037e-1_f64 * t40215;
    let t41749 = 0.21951497276451705328e-1_f64 * t40217;
    let t41751 = 0.46230515946956099004e0_f64 * t40222;
    let t41756 = 0.39029762157531132074e-1_f64 * t40232;
    let t41757 = 0.11708928647259339622e0_f64 * t40234;
    let t41762 = 0.93149212406257582492e-1_f64 * t40241;
    let t41763 = 0.39029762157531132074e-1_f64 * t40243;
    let t41775 = 0.21951497276451705328e-1_f64 * t40257;
    (t41734, t41735, t41736, t41743, t41748, t41749, t41751, t41756, t41757, t41762, t41763, t41775)
}
