//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1018/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1018<F: Float>(t113: F, t2526: F, t808: F, t153: F, t160: F, t2150: F, t2605: F, t2484: F, t7624: F, t7627: F, t815: F, t2491: F) -> (F, F, F, F, F, F, F) {
    let t26533 = t113 * t2526;
    let t26534 = t808 * t26533;
    let t26536 = t153 * t160;
    let t26538 = t2605 * t2150;
    let t26540 = t2484 * t7624;
    let t26542 = t815 * t7627;
    let t26544 = t2150 * t2491;
    (t26533, t26534, t26536, t26538, t26540, t26542, t26544)
}
