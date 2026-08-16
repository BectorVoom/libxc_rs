//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 995/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk995(t10760: f64, t8089: f64, t6535: f64, t261: f64, t2726: f64, t3299: f64, t10730: f64, t10732: f64, t10742: f64, t10744: f64, t10759: f64, t10770: f64, t11672: f64, t11676: f64, t11679: f64, t11681: f64) -> (f64, f64, f64) {
    let t11683 = t10760 * t8089;
    let t11684 = t6535 * t11683;
    let t11686 = t261 * t2726;
    let t11687 = t3299 * t11686;
    let t11689 = 0.23804984598836975486e-2_f64 * t10730 - 0.23804984598836975486e-2_f64 * t10732 - t10742 + 0.12805040077930161442e0_f64 * t10744 + t10759 + 0.16463622957338778997e0_f64 * t11672 + 0.23804984598836975486e-2_f64 * t10770 - 0.65495539973149862688e-2_f64 * t11676 + 0.21831846657716620896e-2_f64 * t11679 - 0.23804984598836975486e-2_f64 * t11681 - 0.43663693315433241792e-2_f64 * t11684 + 0.11557628986739024751e0_f64 * t11687;
    (t11683, t11686, t11689)
}
