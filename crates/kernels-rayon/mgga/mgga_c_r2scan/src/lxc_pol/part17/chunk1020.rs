//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1020/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1020(t11417: f64, t11753: f64, t11758: f64, t11762: f64, t11766: f64, t11768: f64, t11772: f64, t11774: f64, t12512: f64, t12515: f64, t12518: f64, t12521: f64, t12524: f64, t12527: f64, t12530: f64) -> f64 {
    let t12798 = 0.17336443480108537126e0_f64 * t12512 - 0.39029762157531132074e-1_f64 * t11753 + 0.10975748638225852664e-1_f64 * t11758 + 0.93149212406257582492e-1_f64 * t11762 - 0.27944763721877274748e0_f64 * t11766 - 0.19514881078765566037e-1_f64 * t11768 - 0.93149212406257582492e-1_f64 * t11772 + 0.21951497276451705328e-1_f64 * t11774 - 0.17336443480108537126e0_f64 * t12515 - 0.86682217400542685632e-1_f64 * t12518 - 0.86682217400542685632e-1_f64 * t12521 - 0.54878743191129263322e-1_f64 * t12524 - 0.54878743191129263322e-1_f64 * t12527 - t11417 + 0.43663693315433241794e-2_f64 * t12530;
    t12798
}
