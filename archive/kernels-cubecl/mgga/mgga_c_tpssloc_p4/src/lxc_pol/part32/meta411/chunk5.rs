//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1588/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1588<F: Float>(t18211: F, t4900: F, t15382: F, t15390: F, t1171: F, t6109: F, t6011: F, t699: F, t11219: F, t18206: F, t136: F, t3297: F) -> (F, F, F, F, F, F) {
    let t18475 = t4900 * t18211;
    let t18484 = t15390 * t15382;
    let t18489 = t6109 * t1171;
    let t18494 = t699 * t6011;
    let t18496 = t11219 * t18206;
    let t18497 = t136 * t18496;
    let t18499 = t3297 * t18211;
    (t18475, t18484, t18489, t18494, t18497, t18499)
}
