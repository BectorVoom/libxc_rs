//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1497/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1497<F: Float>(t11648: F, t3169: F, t3133: F, t373: F, t1062: F, t11782: F, t10356: F, t11150: F, t357: F, t11853: F, t828: F, t3229: F, t360: F) -> (F, F, F, F, F, F, F) {
    let t42383 = t3169 * t11648;
    let t42385 = t3133 * t3133;
    let t42386 = t373 * t42385;
    let t42391 = t11782 * t1062;
    let t42397 = t357 * t11150 * t10356;
    let t42410 = t828 * t11853;
    let t42415 = t360 * t3229;
    (t42383, t42385, t42386, t42391, t42397, t42410, t42415)
}
