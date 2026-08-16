//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1140/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1140<F: Float>(t40175: F, t40177: F, t40180: F, t40201: F, t40215: F, t40217: F, t40222: F, t40232: F, t40234: F, t40241: F, t40243: F, t40257: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41734 = F::cast_from(0.46230515946956099004e0_f64) * t40175;
    let t41735 = F::cast_from(0.13869154784086829701e1_f64) * t40177;
    let t41736 = F::cast_from(0.13869154784086829701e1_f64) * t40180;
    let t41743 = F::cast_from(0.19043987679069580389e-1_f64) * t40201;
    let t41748 = F::cast_from(0.19514881078765566037e-1_f64) * t40215;
    let t41749 = F::cast_from(0.21951497276451705328e-1_f64) * t40217;
    let t41751 = F::cast_from(0.46230515946956099004e0_f64) * t40222;
    let t41756 = F::cast_from(0.39029762157531132074e-1_f64) * t40232;
    let t41757 = F::cast_from(0.11708928647259339622e0_f64) * t40234;
    let t41762 = F::cast_from(0.93149212406257582492e-1_f64) * t40241;
    let t41763 = F::cast_from(0.39029762157531132074e-1_f64) * t40243;
    let t41775 = F::cast_from(0.21951497276451705328e-1_f64) * t40257;
    (t41734, t41735, t41736, t41743, t41748, t41749, t41751, t41756, t41757, t41762, t41763, t41775)
}
