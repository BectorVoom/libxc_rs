//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1335/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1335<F: Float>(t40097: F, t760: F, t186: F, t2698: F, t685: F, t755: F, t2491: F, t2495: F, t39871: F, t2598: F, t9321: F, t39875: F, t9367: F) -> (F, F, F, F, F, F, F, F) {
    let t40099 = F::cast_from(0.46785788981077169656e1_f64) * t760 * t40097;
    let t40101 = t685 * t2698 * t186;
    let t40103 = F::cast_from(0.18989649058080861537e-2_f64) * t755 * t40101;
    let t40113 = t2491 * t39871 * t2495;
    let t40115 = F::cast_from(0.51947577317044391277e2_f64) * t760 * t40113;
    let t40129 = t9321 * t2598;
    let t40131 = F::cast_from(0.21053605041484726346e2_f64) * t760 * t40129;
    let t40135 = t9367 * t39875 * t2495;
    (t40099, t40101, t40103, t40113, t40115, t40129, t40131, t40135)
}
