//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1106/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1106<F: Float>(t39982: F, t10894: F, t7625: F, t10868: F, t6165: F, t8156: F, t8160: F, t37754: F, t546: F, t38145: F, t6085: F, t7922: F) -> (F, F, F, F, F, F) {
    let t39983 = F::cast_from(0.46230515946956099004e0_f64) * t39982;
    let t39984 = t10894 * t7625;
    let t39985 = F::cast_from(0.54878743191129263322e-2_f64) * t39984;
    let t39995 = t6165 * t10868 * t8156;
    let t39996 = F::cast_from(0.13972381860938637374e0_f64) * t39995;
    let t40001 = t6165 * t10868 * t8160;
    let t40033 = t546 * t37754;
    let t40041 = t6085 * t38145 * t7922;
    (t39983, t39985, t39996, t40001, t40033, t40041)
}
