//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 865/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk865<F: Float>(t4882: F, t5137: F, t639: F, t2735: F, t662: F, t211: F, t5129: F, t5529: F, t587: F, t4972: F, t5125: F, t4963: F, t7669: F) -> (F, F, F, F, F) {
    let t16629 = t639 * t5137 * t4882;
    let t16630 = F::new(64.0) / F::new(45.0) * t16629;
    let t16631 = t2735 * t662;
    let t16632 = t211 * t16631;
    let t16633 = F::new(64.0) / F::new(405.0) * t16632;
    let t16635 = t587 * t5129 * t5529;
    let t16636 = F::new(32.0) / F::new(45.0) * t16635;
    let t16638 = t587 * t5125 * t4972;
    let t16639 = F::new(64.0) / F::new(45.0) * t16638;
    let t16641 = t587 * t7669 * t4963;
    (t16630, t16633, t16636, t16639, t16641)
}
