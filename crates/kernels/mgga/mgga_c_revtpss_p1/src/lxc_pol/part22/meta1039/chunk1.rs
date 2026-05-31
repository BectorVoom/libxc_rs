//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3631/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3631<F: Float>(t16677: F, t5192: F, t1196: F, t12485: F, t3524: F, t6534: F, t20400: F, t3535: F, t17164: F, t20391: F, t3531: F, t3427: F, t3433: F, t6439: F) -> (F, F, F, F, F, F) {
    let t68738 = F::cast_from(0.46785788981077169656e1_f64) * t5192 * t16677;
    let t68742 = F::cast_from(0.10389515463408878255e3_f64) * t1196 * t12485 * t6534 * t3524;
    let t68744 = F::cast_from(0.11696447245269292414e1_f64) * t20400 * t3535;
    let t68746 = F::cast_from(0.11696447245269292414e1_f64) * t5192 * t17164;
    let t68748 = F::cast_from(0.70178683471615754484e1_f64) * t3531 * t20391;
    let t68751 = F::cast_from(6.0_f64) * t3433 * t6439 * t3427;
    (t68738, t68742, t68744, t68746, t68748, t68751)
}
