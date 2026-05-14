//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 873/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk873<F: Float>(t2741: F, t7945: F, t1663: F, t3345: F, t3518: F, t5463: F, t639: F, t3503: F, t4991: F, t587: F, t1672: F, t185: F, t3455: F, t10742: F, t586: F, t1620: F, t16904: F, t3402: F) -> (F, F, F, F, F, F, F) {
    let t30407 = t2741 * t7945;
    let t30455 = t3345 * t1663;
    let t30511 = t639 * t5463 * t3518;
    let t30583 = t587 * t4991 * t3503;
    let t30593 = t185 * t1672 * t3455;
    let t30630 = t10742 * t586;
    let t30660 = t1620 * t16904 * t3402;
    (t30407, t30455, t30511, t30583, t30593, t30630, t30660)
}
