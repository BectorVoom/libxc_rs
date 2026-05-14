//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 501/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk501<F: Float>(t229: F, t2483: F, t595: F, t970: F, t637: F, t406: F, t959: F, t410: F, t697: F, t898: F, t60: F, t955: F) -> (F, F, F, F, F, F, F) {
    let t2755 = t2483 * t229;
    let t2758 = t595 * t970;
    let t2759 = t2758 * t637;
    let t2761 = t406 * t959;
    let t2763 = t410 * t959;
    let t2765 = t898 * t697;
    let t2768 = t60 * t955;
    (t2755, t2758, t2759, t2761, t2763, t2765, t2768)
}
