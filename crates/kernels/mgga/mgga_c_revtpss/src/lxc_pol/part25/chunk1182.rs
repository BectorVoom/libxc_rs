//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1182/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1182<F: Float>(t25895: F, t94604: F, t7243: F, t9292: F, t1032: F, t4066: F, t1955: F, t25878: F, t2453: F, t3908: F, t7275: F, t3923: F, t7274: F, t1399: F, t2434: F, t25880: F) -> (F, F, F, F, F, F, F, F) {
    let t94605 = t25895 * t94604;
    let t94608 = 0.17073386770573548589e-1 * t9292 * t7243;
    let t94609 = t4066 * t1032;
    let t94610 = t1955 * t94609;
    let t94613 = t25878 * t94604;
    let t94616 = t2453 * t7275 * t3908;
    let t94628 = t7274 * t3923;
    let t94633 = t2434 * t1399;
    let t94634 = t25880 * t94633;
    (t94605, t94608, t94609, t94610, t94613, t94616, t94628, t94634)
}
