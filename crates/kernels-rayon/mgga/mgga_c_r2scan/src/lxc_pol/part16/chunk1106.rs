//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1106/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1106(t39982: f64, t10894: f64, t7625: f64, t10868: f64, t6165: f64, t8156: f64, t8160: f64, t37754: f64, t546: f64, t38145: f64, t6085: f64, t7922: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39983 = 0.46230515946956099004e0_f64 * t39982;
    let t39984 = t10894 * t7625;
    let t39985 = 0.54878743191129263322e-2_f64 * t39984;
    let t39995 = t6165 * t10868 * t8156;
    let t39996 = 0.13972381860938637374e0_f64 * t39995;
    let t40001 = t6165 * t10868 * t8160;
    let t40033 = t546 * t37754;
    let t40041 = t6085 * t38145 * t7922;
    (t39983, t39985, t39996, t40001, t40033, t40041)
}
