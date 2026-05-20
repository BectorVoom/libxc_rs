//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1512/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1512<F: Float>(t14648: F, t14832: F, t2661: F, t5962: F, t23346: F, t2652: F, t231: F, t2662: F, t76569: F, t23244: F, t243: F, t10871: F, t40693: F) -> (F, F, F, F, F) {
    let t76812 = t2661 * t14832 * t14648 * t5962;
    let t76814 = t2652 * t23346;
    let t76818 = t2661 * t2662 * t76569 * t231;
    let t76823 = t2661 * t2662 * t243 * t23244 * t231;
    let t76827 = t2661 * t40693 * t76569 * t10871;
    (t76812, t76814, t76818, t76823, t76827)
}
