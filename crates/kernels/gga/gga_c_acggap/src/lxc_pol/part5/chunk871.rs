//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 871/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk871<F: Float>(t435: F, t965: F, t3216: F, t3357: F, t3652: F, t3657: F, t3806: F, t3207: F, t363: F, t1080: F, t987: F, t656: F, t668: F, t682: F) -> (F, F, F, F, F, F, F, F) {
    let t12610 = t965 * t435;
    let t12615 = t3216 * t3357;
    let t12621 = t3216 * t3652;
    let t12623 = t3216 * t3657;
    let t12626 = F::new(0.24009450146119052704e-1) * t3216 * t3806;
    let t12641 = t3207 * t363;
    let t12646 = t987 * t1080;
    let t12661 = F::new(0.43374325201206959368e-1) * t656 * t668 * t682;
    (t12610, t12615, t12621, t12623, t12626, t12641, t12646, t12661)
}
