//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1624/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1624<F: Float>(t3568: F, t3588: F, t12640: F, t1284: F, t3624: F, t127: F, t12866: F, t3630: F, t3712: F, t12809: F, t12811: F, t12916: F) -> (F, F, F, F) {
    let t44618 = t3568 * t3588;
    let t44624 = t12640 * t1284 * t3624;
    let t44634 = t12866 * t127 * t3712 * t3630;
    let t44637 = t12809 * t12916 * t12811;
    (t44618, t44624, t44634, t44637)
}
