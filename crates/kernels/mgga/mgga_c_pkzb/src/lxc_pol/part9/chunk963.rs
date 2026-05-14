//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 963/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk963<F: Float>(t2368: F, t2370: F, t8429: F, t406: F, t3207: F, t8380: F, t2387: F, t394: F, t3186: F, t6456: F, t8427: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8436 = t2370 * t2368;
    let t8437 = t8429 * t8436;
    let t8438 = t406 * t8437;
    let t8441 = t8380 * t3207;
    let t8442 = t406 * t8441;
    let t8445 = t2387 * t394;
    let t8446 = t3186 * t8445;
    let t8447 = t406 * t8446;
    let t8450 = t6456 * t8427;
    (t8436, t8437, t8438, t8441, t8442, t8445, t8446, t8447, t8450)
}
