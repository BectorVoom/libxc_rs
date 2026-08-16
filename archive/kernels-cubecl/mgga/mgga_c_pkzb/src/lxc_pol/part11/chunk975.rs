//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 975/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk975<F: Float>(t2660: F, t2661: F, t3396: F, t10628: F, t5391: F, t592: F, t1020: F, t179: F, t8914: F, t8962: F, t2600: F, t3401: F) -> (F, F, F, F, F) {
    let t10643 = t2660 * t2661 * t3396;
    let t10647 = t592 * t10628 * t5391;
    let t10651 = t179 * t8914 * t1020;
    let t10655 = t179 * t8962 * t1020;
    let t10659 = t179 * t2600 * t3401;
    (t10643, t10647, t10651, t10655, t10659)
}
