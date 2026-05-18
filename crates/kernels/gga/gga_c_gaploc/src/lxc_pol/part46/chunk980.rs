//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 980/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk980<F: Float>(t13055: F, t5640: F, t13058: F, t1991: F, t20671: F, t28309: F, t33601: F, t13023: F, t4614: F, t833: F, t43008: F, t4820: F, t7513: F) -> (F, F, F, F, F) {
    let t43652 = t5640 * t13055;
    let t43653 = F::new(0.15337170381568299871e1) * t43652;
    let t43657 = t1991 * t13058;
    let t43658 = F::new(0.1022478025437886658e1) * t43657;
    let t43660 = t28309 * t20671 * t33601;
    let t43661 = F::new(0.17041300423964777634e0) * t43660;
    let t43664 = F::new(0.15337170381568299871e2) * t833 * t4614 * t13023;
    let t43666 = t7513 * t4820 * t43008;
    (t43653, t43658, t43661, t43664, t43666)
}
