//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1275/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1275<F: Float>(t2575: F, t637: F, t1535: F, t16810: F, t16813: F, t16822: F, t24660: F, t24663: F, t24664: F, t24665: F, t24666: F, t24667: F, t24668: F, t24669: F, t2537: F, t2706: F) -> (F, F) {
    let t25047 = t2575 * t637;
    let t25051 = -12.0 * t1535 * t25047 * t2537 + t16810 - t16813 - t16822 - t24660 - t24663 + t24664 - t24665 - t24666 + t24667 + t24668 - t24669;
    let t25058 = t637 * t2706;
    (t25051, t25058)
}
