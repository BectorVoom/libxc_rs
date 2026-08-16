//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1298/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1298<F: Float>(t1679: F, t2049: F, t582: F, t7682: F, t1982: F, t1981: F, t1993: F, t19050: F, t546: F, t116: F, t18679: F, t18363: F, t5791: F) -> (F, F, F, F, F, F, F) {
    let t61942 = t1679 * t2049;
    let t62007 = t7682 * t582;
    let t62020 = t1679 * t1982;
    let t62024 = t1981 * t1993;
    let t62171 = t546 * t19050;
    let t62230 = t18679 * t116;
    let t62247 = t18363 * t5791;
    (t61942, t62007, t62020, t62024, t62171, t62230, t62247)
}
