//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 995/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk995<F: Float>(t10760: F, t8089: F, t6535: F, t261: F, t2726: F, t3299: F, t10730: F, t10732: F, t10742: F, t10744: F, t10759: F, t10770: F, t11672: F, t11676: F, t11679: F, t11681: F) -> (F, F, F) {
    let t11683 = t10760 * t8089;
    let t11684 = t6535 * t11683;
    let t11686 = t261 * t2726;
    let t11687 = t3299 * t11686;
    let t11689 = F::cast_from(0.23804984598836975486e-2_f64) * t10730 - F::cast_from(0.23804984598836975486e-2_f64) * t10732 - t10742 + F::cast_from(0.12805040077930161442e0_f64) * t10744 + t10759 + F::cast_from(0.16463622957338778997e0_f64) * t11672 + F::cast_from(0.23804984598836975486e-2_f64) * t10770 - F::cast_from(0.65495539973149862688e-2_f64) * t11676 + F::cast_from(0.21831846657716620896e-2_f64) * t11679 - F::cast_from(0.23804984598836975486e-2_f64) * t11681 - F::cast_from(0.43663693315433241792e-2_f64) * t11684 + F::cast_from(0.11557628986739024751e0_f64) * t11687;
    (t11683, t11686, t11689)
}
