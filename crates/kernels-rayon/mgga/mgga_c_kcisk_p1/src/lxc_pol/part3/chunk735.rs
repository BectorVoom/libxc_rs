//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 735/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk735(t5101: f64, t707: f64, t1824: f64, t4658: f64, t706: f64, t1797: f64, t180: f64, t479: f64, t574: f64, t682: f64, t695: f64, t1060: f64, t1648: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11393 = t707 * t5101;
    let t11394 = t4658 * t1824;
    let t11395 = t11393 * t11394;
    let t11396 = t706 * t11395;
    let t11400 = t180 * t479 * t1797;
    let t11401 = t574 * t682;
    let t11402 = t11401 * t695;
    let t11403 = t1060 * t1648;
    (t11394, t11395, t11396, t11400, t11402, t11403)
}
