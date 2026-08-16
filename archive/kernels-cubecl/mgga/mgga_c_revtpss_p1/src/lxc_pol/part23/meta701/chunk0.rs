//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2451/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2451<F: Float>(t4038: F, t9425: F, t1330: F, t512: F, t9544: F, t3869: F, t39739: F, t39430: F, t9572: F, t9860: F, t39742: F, t39440: F) -> (F, F, F, F, F, F, F) {
    let t47110 = t4038 * t9425;
    let t47113 = t512 * t1330 * t9544;
    let t47116 = F::cast_from(0.86748650402413918736e-1_f64) * t3869 * t39739;
    let t47118 = F::cast_from(0.38527786510141256862e1_f64) * t3869 * t39430;
    let t47119 = t9860 * t9572;
    let t47122 = F::cast_from(0.1301229756036208781e0_f64) * t3869 * t39742;
    let t47124 = F::cast_from(0.67471172535210825684e-1_f64) * t3869 * t39440;
    (t47110, t47113, t47116, t47118, t47119, t47122, t47124)
}
