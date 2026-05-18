//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1085/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1085<F: Float>(t11661: F, t276: F, t40: F, t5474: F, t11679: F, t11681: F, t14880: F, t14883: F, t14885: F, t14890: F, t11665: F, t11668: F, t11672: F, t3984: F, t6614: F, t694: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19425 = F::new(0.32530743900905219526e-1) * t11661;
    let t19430 = t40 * t5474 * t276;
    let t19431 = F::new(2.0) * t19430;
    let t19432 = F::new(48.0) * t11679;
    let t19433 = F::new(96.0) * t11681;
    let t19434 = F::new(0.96319466275353142155e0) * t14880;
    let t19435 = F::new(0.43374325201206959368e-1) * t14883;
    let t19436 = F::new(0.32530743900905219526e-1) * t14885;
    let t19437 = F::new(0.43374325201206959368e-1) * t14890;
    let t19438 = -F::new(6.0) * t3984 * t6614 * t694 + t11665 + t11668 - t11672 + t19425 + t19431 + t19432 + t19433 + t19434 + t19435 + t19436 - t19437;
    (t19425, t19431, t19432, t19433, t19434, t19435, t19436, t19437, t19438)
}
