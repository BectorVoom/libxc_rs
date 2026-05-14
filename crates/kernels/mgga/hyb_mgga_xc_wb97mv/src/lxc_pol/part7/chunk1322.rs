//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1322/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1322<F: Float>(t27393: F, t27396: F, t27488: F, t27550: F, t27555: F, t27578: F, t27585: F, t27652: F, t27657: F, t32047: F, t32049: F, t32060: F, t3568: F, t3601: F, t9458: F, t9501: F, t9508: F, t9511: F, t9515: F, t9524: F, t9527: F, t9530: F, t9534: F, t9537: F, t9541: F, t9554: F, t9558: F, t9561: F, t9565: F, t9568: F, t994: F) -> (F,) {
    let t32391 = -8.0 * t9508 * t9524 - 4.0 * t9508 * t9527 - 0.38596750796862084161e3 * t27396 * t9530 + 0.12865583598954028054e3 * t27550 * t3568 + 0.12865583598954028054e3 * t9501 * t9534 + 0.64327917994770140268e2 * t9501 * t9537 + 0.4138081033541872024e4 * t27555 * t9541 - 0.46785788981077169656e1 * t9511 * t9554 - 0.23392894490538584828e1 * t9511 * t9558 - 0.2077903092681775651e3 * t27393 * t9561 + 0.69263436422725855034e2 * t27652 * t3601 + 0.69263436422725855034e2 * t9458 * t9565 + 0.34631718211362927517e2 * t9458 * t9568 + 0.20508037716432813315e4 * t27657 * t9515 - t32047 - 0.77193501593724168323e3 * t27585 * t9534 + t32049 + t32060 + 0.41016075432865626631e4 * t27578 * t27488 * t994;
    (t32391,)
}
