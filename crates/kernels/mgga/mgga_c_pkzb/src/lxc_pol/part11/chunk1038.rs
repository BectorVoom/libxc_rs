//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1038/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1038<F: Float>(t10344: F, t10356: F, t11483: F, t11501: F, t11507: F, t11510: F, t11520: F, t11524: F, t11527: F, t1250: F, t3259: F, t3273: F, t3914: F, t3920: F, t3923: F, t397: F, t6555: F, t6569: F, t6590: F, t8546: F, t8554: F, t943: F) -> F {
    let t11532 = F::new(0.39512695097613069591e1) * t6555 * t11501 + F::new(0.39512695097613069591e1) * t8546 * t3914 + F::new(0.39512695097613069591e1) * t3259 * t11507 - F::new(0.39512695097613069591e1) * t6569 * t11510 + F::new(0.19756347548806534796e1) * t10356 * t1250 + F::new(0.19756347548806534796e1) * t3273 * t3920 - F::new(0.19756347548806534796e1) * t8554 * t3923 + F::new(0.65854491829355115987e0) * t943 * t11520 - F::new(0.19756347548806534796e1) * t10344 * t11524 + F::new(0.65854491829355115987e0) * t6590 * t11527 + F::new(0.65854491829355115987e0) * t397 * t11483;
    t11532
}
