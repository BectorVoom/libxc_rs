//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 743/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk743<F: Float>(t4137: F, t557: F, t1938: F, t857: F, t1782: F, t360: F, t3300: F, t398: F, t372: F, t5011: F, t1524: F, t513: F) -> (F, F, F, F, F) {
    let t5523 = t4137 * t557;
    let t5525 = t857 * t1938;
    let t5527 = t1782 * t360;
    let t5529 = t398 * t3300 * t5527;
    let t5532 = t1782 * t372;
    let t5534 = t398 * t5011 * t5532;
    let t5537 = t513 * t1524;
    (t5523, t5525, t5529, t5534, t5537)
}
