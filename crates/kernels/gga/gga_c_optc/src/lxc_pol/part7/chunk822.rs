//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 822/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk822<F: Float>(t7777: F, t7792: F, t818: F, t2516: F, t808: F, t243: F, t251: F, t2519: F, t7747: F, t2427: F, t828: F, t2472: F, t824: F) -> (F, F, F, F, F, F, F, F) {
    let t7793 = t7777 + t7792;
    let t7794 = t7793 * t818;
    let t7798 = F::cast_from(1.0_f64) / t2516 / t808;
    let t7799 = t243 * t7798;
    let t7801 = F::cast_from(1.0_f64) / t2519 / t251;
    let t7802 = t7747 * t7801;
    let t7805 = t2427 * t828;
    let t7810 = t824 * t2472;
    (t7793, t7794, t7798, t7799, t7801, t7802, t7805, t7810)
}
