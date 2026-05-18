//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 764/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk764<F: Float>(t1865: F, t2667: F, t7226: F, t2717: F, t702: F, t1836: F, t954: F, t2060: F, t937: F, t2532: F, t779: F, t1710: F, t2581: F) -> (F, F, F, F, F, F, F) {
    let t7227 = t2667 * t1865;
    let t7228 = t7226 * t7227;
    let t7233 = t2717 * t702;
    let t7236 = t954 * t1836;
    let t7239 = t2060 * t937;
    let t7242 = t779 * t2532;
    let t7245 = t2581 * t1710;
    (t7227, t7228, t7233, t7236, t7239, t7242, t7245)
}
