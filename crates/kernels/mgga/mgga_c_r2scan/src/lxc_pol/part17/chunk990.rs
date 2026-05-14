//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 990/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk990<F: Float>(t10894: F, t7625: F, t10868: F, t6165: F, t8156: F, t8160: F, t37754: F, t546: F, t38145: F, t6085: F, t7922: F, t6093: F, t7605: F, t8081: F, t7619: F, t2147: F, t7624: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39984 = t10894 * t7625;
    let t39995 = t6165 * t10868 * t8156;
    let t40001 = t6165 * t10868 * t8160;
    let t40033 = t546 * t37754;
    let t40041 = t6085 * t38145 * t7922;
    let t40044 = t6093 * t38145 * t7605;
    let t40047 = t6085 * t38145 * t8081;
    let t40050 = t6093 * t38145 * t7619;
    let t40053 = t2147 * t38145 * t7624;
    (t39984, t39995, t40001, t40033, t40041, t40044, t40047, t40050, t40053)
}
