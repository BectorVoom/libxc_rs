//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 751/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk751<F: Float>(t495: F, t560: F, t2541: F, t1734: F, t469: F, t1814: F, t609: F, t944: F, t7890: F, t1914: F, t8004: F, t157: F, t2152: F, t524: F, t556: F, t7932: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9476 = t495 * t560;
    let t9477 = t2541 * t9476;
    let t9480 = t469 * t1734;
    let t9491 = t609 * t1814;
    let t9492 = t9491 * t944;
    let t9493 = t7890 * t9492;
    let t9497 = t609 * t1914;
    let t9498 = t8004 * t9497;
    let t9502 = t9491 * t157;
    let t9503 = t2152 * t9502;
    let t9508 = t556 * t524 * t157;
    let t9509 = t7932 * t9508;
    (t9476, t9477, t9480, t9493, t9497, t9498, t9503, t9508, t9509)
}
