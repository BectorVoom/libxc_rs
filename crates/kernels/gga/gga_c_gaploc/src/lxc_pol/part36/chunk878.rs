//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 878/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk878<F: Float>(t10105: F, t2969: F, t11127: F, t7324: F, t3511: F, t7822: F, t13235: F, t16710: F, t841: F, t11125: F, t2592: F, t2728: F, t3459: F, t5559: F, t10800: F, t1960: F) -> (F, F, F, F, F, F, F, F) {
    let t44194 = t2969 * t10105;
    let t44196 = t7324 * t11127;
    let t44198 = t7822 * t3511;
    let t44202 = 24.0 * t16710 * t13235 * t841;
    let t44203 = t2592 * t11125;
    let t44207 = 12.0 * t5559 * t3459 * t2728;
    let t44208 = t10800 * t2728;
    let t44211 = t1960 * t3511 * t2728;
    (t44194, t44196, t44198, t44202, t44203, t44207, t44208, t44211)
}
