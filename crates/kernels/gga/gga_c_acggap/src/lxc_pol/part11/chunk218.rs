//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 218/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk218<F: Float>(t752: F, t132: F, t200: F, t220: F, t721: F, t199: F, t27: F, t13: F, t218: F, t219: F, t663: F, t666: F, t669: F, t673: F, t675: F, t678: F) -> (F, F, F, F, F, F, F, F, F) {
    let t753 = 0.14764627977777777777e-2 * t752;
    let t754 = t132 * t200;
    let t756 = t721 * t754 * t220;
    let t757 = 0.35616666666666666666e-1 * t756;
    let t758 = t199 * t27;
    let t759 = 1.0 / t758;
    let t760 = t13 * t759;
    let t761 = t218 * t218;
    let t762 = t761 * t219;
    let t763 = t760 * t762;
    let t764 = 2.0 * t763;
    let t771 = -0.42198333333333333333e0 * t663 + 0.84396666666666666666e0 * t666 + 0.39862222222222222223e0 * t669 + 0.68258333333333333333e-1 * t673 + 0.13651666666666666667e0 * t675 + 0.13692777777777777778e0 * t678;
    (t753, t754, t757, t759, t760, t761, t762, t764, t771)
}
