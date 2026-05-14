//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1096/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1096<F: Float>(t27114: F, t375: F, t89: F, t27092: F, t376: F, t5890: F, t1637: F, t6657: F, t1636: F, t6681: F, t27124: F, t1369: F, t27053: F, t27131: F, t1882: F, t27106: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t105685 = t89 * t375 * t27114;
    let t105686 = 2.0 / 3.0 * t105685;
    let t105696 = t5890 * t376 * t27092;
    let t105697 = t105696 / 6.0;
    let t105711 = t5890 * t1637 * t6657;
    let t105733 = t89 * t1636 * t6681;
    let t105740 = t5890 * t376 * t27124;
    let t105741 = t105740 / 6.0;
    let t105743 = t1369 * t376 * t27053;
    let t105744 = 2.0 / 3.0 * t105743;
    let t105760 = t1369 * t376 * t27131;
    let t105761 = 2.0 / 3.0 * t105760;
    let t105765 = t1882 * t27106;
    (t105685, t105686, t105696, t105697, t105711, t105733, t105740, t105741, t105743, t105744, t105760, t105761, t105765)
}
