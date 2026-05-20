//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2765/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2765<F: Float>(t4398: F, t9323: F, t39989: F, t40137: F, t40141: F, t50065: F, t50070: F, t50085: F, t50091: F, t50093: F, t50095: F, t50096: F, t50098: F, t50100: F, t50101: F, t50106: F, t50114: F, t50115: F) -> (F, F) {
    let t50852 = t4398 * t9323;
    let t50853 = F::cast_from(0.51947577317044391277e2_f64) * t50852;
    let t50854 = -t50065 - t40137 + t50070 + t50085 + t50091 + t50093 + t50095 + t40141 + t50096 + t50098 + t50100 + t50101 + t50106 - t39989 + t50114 + t50115 - t50853;
    (t50853, t50854)
}
