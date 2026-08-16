//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2411/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2411<F: Float>(t10021: F, t812: F, t841: F, t849: F, t23076: F, t241: F, t67: F, t2707: F, t9601: F, t2703: F, t2559: F, t2570: F) -> (F, F, F, F, F, F) {
    let t40965 = t812 * t841 * t10021;
    let t40966 = t40965 * t849;
    let t40971 = t241 * t23076 * t67;
    let t40982 = t9601 * t2707;
    let t40990 = t9601 * t2703;
    let t41008 = t2559 * t2570;
    (t40965, t40966, t40971, t40982, t40990, t41008)
}
