//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1317/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1317<F: Float>(t2186: F, t3946: F, t626: F, t9365: F, t29904: F, t29895: F, t29908: F, t29900: F, t29927: F, t45435: F, t64: F, t614: F, t96: F) -> (F, F, F, F, F, F, F) {
    let t110032 = t2186 * t3946;
    let t110075 = t626 * t9365;
    let t110076 = t110075 * t29904;
    let t110078 = t29895 * t29908;
    let t110080 = t29900 * t29927;
    let t110082 = t64 * t45435;
    let t110089 = t614 * t96;
    (t110032, t110075, t110076, t110078, t110080, t110082, t110089)
}
