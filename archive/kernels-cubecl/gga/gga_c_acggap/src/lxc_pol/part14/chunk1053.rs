//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1053/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1053<F: Float>(t1679: F, t1717: F, t9097: F, t1814: F, t2122: F, t33489: F, t7942: F, t8406: F, t157: F, t1914: F, t406: F, t33796: F, t9030: F) -> (F, F, F, F, F) {
    let t38615 = t1679 * t9097 * t1717;
    let t38621 = t2122 * t1814;
    let t38631 = t7942 * t33489 * t8406;
    let t38635 = t1914 * t406 * t157;
    let t38639 = t33796 * t9030;
    (t38615, t38621, t38631, t38635, t38639)
}
