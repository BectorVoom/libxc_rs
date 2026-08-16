//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 800/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk800(t1333: f64, t8667: f64, t1772: f64, t8793: f64, t2448: f64, t7218: f64, t7208: f64, t7230: f64, t7219: f64, t1769: f64, t8794: f64, t10798: f64, t8797: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23320 = t1333 * t8667;
    let t23326 = t8793 * t1772;
    let t23338 = t2448 * t7218;
    let t23341 = t7208 * t7230;
    let t23344 = t7219 * t7230;
    let t23413 = t8794 * t1769;
    let t23415 = t10798 * t8797;
    (t23320, t23326, t23338, t23341, t23344, t23413, t23415)
}
