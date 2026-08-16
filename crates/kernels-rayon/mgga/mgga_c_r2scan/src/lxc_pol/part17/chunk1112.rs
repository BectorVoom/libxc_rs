//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1112/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1112(t10894: f64, t7625: f64, t10868: f64, t6165: f64, t8156: f64, t8160: f64, t37754: f64, t546: f64, t38145: f64, t6085: f64, t7922: f64, t6093: f64, t7605: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39984 = t10894 * t7625;
    let t39995 = t6165 * t10868 * t8156;
    let t40001 = t6165 * t10868 * t8160;
    let t40033 = t546 * t37754;
    let t40041 = t6085 * t38145 * t7922;
    let t40044 = t6093 * t38145 * t7605;
    (t39984, t39995, t40001, t40033, t40041, t40044)
}
