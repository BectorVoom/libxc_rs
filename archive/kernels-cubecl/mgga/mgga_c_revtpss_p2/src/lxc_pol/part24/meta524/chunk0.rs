//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1555/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1555<F: Float>(t21272: F, t5378: F, t12772: F, t24793: F, t3625: F, t24803: F, t44425: F, t1208: F, t24697: F, t225: F, t480: F, t17438: F, t20846: F) -> (F, F, F, F, F, F, F) {
    let t83018 = t21272 * t5378;
    let t83047 = t3625 * t12772 * t24793;
    let t83067 = t3625 * t44425 * t24803;
    let t83107 = t24697 * t1208;
    let t83108 = t83107 * t225;
    let t83109 = t83108 * t480;
    let t83112 = t17438 * t20846;
    (t83018, t83047, t83067, t83107, t83108, t83109, t83112)
}
