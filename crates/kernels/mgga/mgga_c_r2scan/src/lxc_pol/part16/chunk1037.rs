//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1037/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1037<F: Float>(t10615: F, t12383: F, t3275: F, t10776: F, t10810: F, t3115: F, t3295: F, t9540: F, t9517: F, t3308: F, t37965: F, t8821: F, t37961: F, t9501: F, t1577: F, t9508: F) -> (F, F, F, F, F, F, F) {
    let t42976 = 5.0 / 8.0 * t3275 * t10615 * t12383;
    let t42978 = t10776 * t10810 * t3115;
    let t42980 = t3295 * t9540;
    let t42982 = t3295 * t9517;
    let t42985 = t37965 * t3308 * t8821;
    let t42988 = t37961 * t3308 * t9501;
    let t42991 = t1577 * t3308 * t9508;
    (t42976, t42978, t42980, t42982, t42985, t42988, t42991)
}
