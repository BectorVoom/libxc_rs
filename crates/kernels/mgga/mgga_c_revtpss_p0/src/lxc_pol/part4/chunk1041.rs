//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1041/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1041<F: Float>(t11132: F, t2942: F, t941: F, t2986: F, t960: F, t2979: F, t300: F, t1034: F, t3154: F, t357: F, t3129: F, t3172: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11534 = F::cast_from(0.55403703703703703703e-1_f64) * t11132;
    let t11548 = t941 * t2942;
    let t11554 = t960 * t2986;
    let t11560 = F::cast_from(0.28842592592592592592e-1_f64) * t11132;
    let t11574 = F::cast_from(0.53272592592592592592e-1_f64) * t11132;
    let t11591 = t300 * t2979;
    let t11626 = t1034 * t1034;
    let t11627 = F::new(1.0) / t11626;
    let t11631 = t3154 * t357;
    let t11643 = t3172 * t3129;
    (t11534, t11548, t11554, t11560, t11574, t11591, t11627, t11631, t11643)
}
