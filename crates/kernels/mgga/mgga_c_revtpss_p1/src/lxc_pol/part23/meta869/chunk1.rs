//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2767/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2767<F: Float>(t22212: F, t2626: F, t1320: F, t22195: F, t221: F, t22253: F, t4018: F, t4019: F, t125: F, t21969: F, t1399: F, t6883: F, t9816: F, t9818: F) -> (F, F, F, F, F) {
    let t74130 = t22212 * t2626;
    let t74132 = t1320 * t22195;
    let t74174 = t4018 * t4019 * t221 * t22253;
    let t74177 = t125 * t21969;
    let t74184 = t9816 * t9818 * t6883 * t1399;
    (t74130, t74132, t74174, t74177, t74184)
}
