//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1093/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1093<F: Float>(t4146: F, t10308: F, t1466: F, t2246: F, t5812: F, t1513: F, t5915: F, t116: F, t22746: F, t14586: F, t6016: F, t1558: F, t231: F, t221: F, t23279: F, t22648: F, t602: F) -> (F, F, F, F, F, F, F, F, F) {
    let t47671 = t4146 * t4146;
    let t47672 = 1.0 / t47671;
    let t60224 = t1466 * t10308;
    let t60673 = t5812 * t2246;
    let t75833 = t1513 * t5915;
    let t75941 = t22746 * t116;
    let t76106 = t14586 * t6016;
    let t76161 = t6016 * t1558 * t231;
    let t76613 = t221 * t23279;
    let t85037 = t22648 * t602;
    (t47672, t60224, t60673, t75833, t75941, t76106, t76161, t76613, t85037)
}
