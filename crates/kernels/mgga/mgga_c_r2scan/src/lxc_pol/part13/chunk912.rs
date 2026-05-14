//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 912/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk912<F: Float>(t2124: F, t7406: F, t11670: F, t10760: F, t7619: F, t6093: F, t7624: F, t2147: F, t3344: F, t980: F, t8089: F, t6535: F, t261: F, t2726: F, t3299: F, t10730: F, t10732: F, t10742: F, t10744: F, t10759: F, t10770: F) -> (F, F, F, F, F, F) {
    let t11671 = t2124 * t7406;
    let t11672 = t11670 * t11671;
    let t11675 = t10760 * t7619;
    let t11676 = t6093 * t11675;
    let t11678 = t10760 * t7624;
    let t11679 = t2147 * t11678;
    let t11681 = t980 * t3344;
    let t11683 = t10760 * t8089;
    let t11684 = t6535 * t11683;
    let t11686 = t261 * t2726;
    let t11687 = t3299 * t11686;
    let t11689 = 0.23804984598836975486e-2 * t10730 - 0.23804984598836975486e-2 * t10732 - t10742 + 0.12805040077930161442e0 * t10744 + t10759 + 0.16463622957338778997e0 * t11672 + 0.23804984598836975486e-2 * t10770 - 0.65495539973149862688e-2 * t11676 + 0.21831846657716620896e-2 * t11679 - 0.23804984598836975486e-2 * t11681 - 0.43663693315433241792e-2 * t11684 + 0.11557628986739024751e0 * t11687;
    (t11671, t11675, t11678, t11683, t11686, t11689)
}
