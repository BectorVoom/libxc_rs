//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 886/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk886<F: Float>(t11686: F, t3299: F, t10730: F, t10732: F, t10742: F, t10744: F, t10759: F, t10770: F, t11672: F, t11676: F, t11679: F, t11681: F, t11684: F, t2593: F, t3295: F, t2599: F, t3308: F) -> (F, F, F, F) {
    let t11687 = t3299 * t11686;
    let t11689 = 0.23804984598836975486e-2 * t10730 - 0.23804984598836975486e-2 * t10732 - t10742 + 0.12805040077930161442e0 * t10744 + t10759 + 0.16463622957338778997e0 * t11672 + 0.23804984598836975486e-2 * t10770 - 0.65495539973149862688e-2 * t11676 + 0.21831846657716620896e-2 * t11679 - 0.23804984598836975486e-2 * t11681 - 0.43663693315433241792e-2 * t11684 + 0.11557628986739024751e0 * t11687;
    let t11691 = t3295 * t2593;
    let t11693 = t3308 * t2599;
    (t11687, t11689, t11691, t11693)
}
