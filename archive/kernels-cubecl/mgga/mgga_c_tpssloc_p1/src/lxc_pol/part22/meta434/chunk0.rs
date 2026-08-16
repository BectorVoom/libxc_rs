//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1770/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1770<F: Float>(t5456: F, t649: F, t5465: F, t626: F, t5464: F, t9365: F, t666: F, t4043: F, t4067: F, t5489: F, t2331: F, t5488: F) -> (F, F, F, F, F, F, F) {
    let t19461 = t649 * t5456;
    let t19471 = t626 * t5465;
    let t19473 = t9365 * t5464;
    let t19474 = t19473 * t666;
    let t19477 = t4043 * t4067;
    let t19480 = t626 * t5489;
    let t19482 = t2331 * t5488;
    (t19461, t19471, t19473, t19474, t19477, t19480, t19482)
}
