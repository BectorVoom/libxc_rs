//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 232/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk232<F: Float>(t301: F, t759: F, t761: F, t758: F, t2: F, t291: F, t3: F) -> (F, F, F) {
    let t762 = t301 * t759 * t761;
    let t763 = t758 * t762;
    let t766 = t291 * t2;
    let t768 = 1.0 / t3 / t766;
    (t762, t763, t768)
}
