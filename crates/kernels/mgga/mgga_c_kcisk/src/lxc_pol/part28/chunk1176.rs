//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1176/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1176<F: Float>(t6714: F, t9665: F, t7246: F, t23819: F, t33021: F, t7261: F, t1333: F, t9960: F, t32942: F, t32956: F, t32987: F, t32990: F, t34113: F, t34116: F, t34119: F, t34122: F, t34125: F, t9664: F, t9667: F, t9936: F) -> (F, F, F, F, F, F) {
    let t34132 = t9665 * t6714;
    let t34133 = t7246 * t34132;
    let t34136 = t33021 * t23819;
    let t34137 = t7261 * t34136;
    let t34140 = t1333 * t9960;
    let t34143 = -0.11574074074074074074e-2 * t32956 - 0.34722222222222222223e-2 * t34113 + 0.11054629629629629629e-2 * t34116 - 0.11574074074074074074e-2 * t34119 - 0.34722222222222222223e-2 * t34122 * t9667 + 0.92592592592592592597e-2 * t34125 * t9667 - 0.34722222222222222223e-2 * t32942 * t9936 - 0.34722222222222222223e-2 * t32990 * t9936 + 0.69444444444444444446e-2 * t9664 * t34133 - 0.20833333333333333334e-1 * t9664 * t34137 + 0.11054629629629629629e-2 * t34140 - 0.16581944444444444444e-2 * t32987;
    (t34132, t34133, t34136, t34137, t34140, t34143)
}
