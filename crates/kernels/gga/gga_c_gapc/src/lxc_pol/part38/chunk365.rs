//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 365/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk365<F: Float>(t1734: F, t1736: F, t1030: F, t1672: F, t6: F, t1688: F, t116: F, t186: F) -> (F, F, F, F) {
    let t1737 = t1734 * t1736;
    let t1738 = t1030 * t1737;
    let t1739 = t1672 * t6;
    let t1740 = t1688 * t1739;
    let t1743 = t116 * t186;
    (t1737, t1738, t1740, t1743)
}
