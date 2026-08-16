//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1094/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1094(t24916: f64, t37945: f64, t37949: f64, t10810: f64, t574: f64, t8066: f64, t10697: f64, t11669: f64, t11671: f64, t10698: f64, t11702: f64, t10885: f64, t11744: f64) -> (f64, f64, f64, f64, f64) {
    let t39485 = t37949 * t37945 * t24916;
    let t39499 = t574 * t10810 * t8066;
    let t39500 = 0.23115257973478049502e0_f64 * t39499;
    let t39502 = t10697 * t11669 * t11671;
    let t39503 = 0.76830240467580968652e0_f64 * t39502;
    let t39511 = t10698 * t11702;
    let t39512 = 0.12805040077930161442e0_f64 * t39511;
    let t39522 = t11744 * t10885;
    (t39485, t39500, t39503, t39512, t39522)
}
