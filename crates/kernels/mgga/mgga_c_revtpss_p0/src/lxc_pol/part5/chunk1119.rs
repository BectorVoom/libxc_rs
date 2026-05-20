//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1119/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1119<F: Float>(t16712: F, t300: F, t5155: F, t16710: F, t16708: F, t1130: F, t5060: F, t1719: F, t3432: F, t5101: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16713 = F::cast_from(0.9877777777777777778e-2_f64) * t16712;
    let t16784 = t300 * t5155;
    let t16797 = F::cast_from(0.23744444444444444444e-1_f64) * t16710;
    let t16798 = F::cast_from(0.11872222222222222222e-1_f64) * t16712;
    let t16820 = F::cast_from(0.41203703703703703704e-2_f64) * t16708;
    let t16821 = F::cast_from(0.12361111111111111111e-1_f64) * t16710;
    let t16822 = F::cast_from(0.61805555555555555556e-2_f64) * t16712;
    let t16835 = t5060 * t1130;
    let t16840 = t1719 * t3432;
    let t16868 = t698 * t5101;
    let t16869 = F::cast_from(0.10954222222222222222e0_f64) * t16868;
    let t16873 = F::cast_from(0.19931111111111111111e0_f64) * t16712;
    (t16713, t16784, t16797, t16798, t16820, t16821, t16822, t16835, t16840, t16868, t16869, t16873)
}
