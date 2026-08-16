//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2867/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2867<F: Float>(t1568: F, t6016: F, t231: F, t2782: F, t2783: F, t2723: F, t4503: F, t76169: F, t14568: F, t18726: F, t10871: F, t14545: F) -> (F, F, F, F, F) {
    let t77159 = t1568 * t6016;
    let t77171 = t2782 * t2783 * t77159 * t231;
    let t77177 = t2782 * t4503 * t76169 * t2723;
    let t77183 = t14568 * t18726;
    let t77191 = t2782 * t14545 * t76169 * t10871;
    (t77159, t77171, t77177, t77183, t77191)
}
