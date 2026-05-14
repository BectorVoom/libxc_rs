//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 847/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk847<F: Float>(t43657: F, t20671: F, t28309: F, t33601: F, t13023: F, t4614: F, t833: F, t43008: F, t4820: F, t7513: F, t43199: F, t2028: F, t3038: F, t787: F, t9641: F, t10999: F, t2536: F) -> (F, F, F, F, F, F, F) {
    let t43658 = 0.1022478025437886658e1 * t43657;
    let t43660 = t28309 * t20671 * t33601;
    let t43661 = 0.17041300423964777634e0 * t43660;
    let t43664 = 0.15337170381568299871e2 * t833 * t4614 * t13023;
    let t43666 = t7513 * t4820 * t43008;
    let t43670 = 0.79445533226334281487e-1 * t7513 * t4820 * t43199;
    let t43674 = 0.39722766613167140743e-1 * t787 * t9641 * t3038 * t2028;
    let t43677 = t787 * t2536 * t10999 * t2028;
    (t43658, t43661, t43664, t43666, t43670, t43674, t43677)
}
