//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 646/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk646<F: Float>(t13078: F, t13119: F, t11849: F, t959: F, t11823: F, t7785: F, t13559: F, t531: F, t13525: F, t808: F, t568: F, t12693: F, t12706: F, t13121: F, t13143: F, t13147: F, t13151: F, t797: F, t813: F) -> (F, F, F, F) {
    let t13697 = 0.59584149919750711116e-1 * t13078;
    let t13700 = 0.11916829983950142223e0 * t13119;
    let t13702 = t11849 * t959;
    let t13703 = 0.14896037479937677779e-1 * t13702;
    let t13704 = t11823 * t7785;
    let t13706 = t531 * t13559;
    let t13709 = t808 * t13525;
    let t13710 = t568 * t13709;
    let t13716 = t13697 - 0.63904876589867916126e-1 * t12693 + 0.63904876589867916126e-1 * t12706 + t13700 + 0.59584149919750711116e-1 * t13121 + t13703 - 0.44688112439813033338e-1 * t13704 - 0.35750489951850426669e0 * t797 * t13706 - 0.23005755572352449806e1 * t813 * t13710 + 0.63904876589867916128e-1 * t13143 - 0.59584149919750711116e-1 * t13147 - 0.63904876589867916128e-1 * t13151;
    (t13706, t13709, t13710, t13716)
}
