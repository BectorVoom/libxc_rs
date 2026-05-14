//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 702/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk702<F: Float>(t20700: F, t6710: F, t9438: F, t12535: F, t1407: F, t20551: F, t6914: F, t12531: F, t587: F, t589: F, t12526: F, t21373: F, t30301: F, t544: F, t9287: F, t12532: F, t7014: F) -> (F, F, F, F, F, F, F, F) {
    let t40372 = t6710 * t9438 * t20700;
    let t40374 = t1407 * t12535;
    let t40377 = t6914 * t9438 * t20551;
    let t40380 = t587 * t589 * t12531;
    let t40392 = t6914 * t21373 * t12526;
    let t40394 = t544 * t30301;
    let t40395 = t40394 * t9287;
    let t40397 = t7014 * t12532;
    (t40372, t40374, t40377, t40380, t40392, t40394, t40395, t40397)
}
