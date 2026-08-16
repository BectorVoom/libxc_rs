//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2436/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2436<F: Float>(t12025: F, t3127: F, t3172: F, t3105: F, t3196: F, t11643: F, t11656: F, t11648: F, t3124: F, t1041: F, t11622: F, t12021: F, t3173: F) -> (F, F, F, F, F, F) {
    let t42193 = t3127 * t3172 * t12025;
    let t42195 = t3196 * t3105;
    let t42204 = t11656 * t11643;
    let t42227 = t3124 * t11648;
    let t42230 = t1041 * t3172 * t11622;
    let t42232 = t12021 * t3173;
    (t42193, t42195, t42204, t42227, t42230, t42232)
}
