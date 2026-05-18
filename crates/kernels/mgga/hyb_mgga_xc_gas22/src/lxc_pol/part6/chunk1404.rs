//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1404/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1404<F: Float>(t10: F, t1107: F, t11217: F, t11245: F, t2707: F, t221: F, t2631: F, t4475: F, t11232: F, t2676: F, t2649: F, t1052: F, t11237: F) -> (F, F, F, F, F, F) {
    let t30390 = t11217 * t10 * t1107;
    let t30392 = t11245 * t2707;
    let t30399 = t4475 * t221 * t2631;
    let t30402 = t11232 * t2676;
    let t30404 = t11232 * t2649;
    let t30406 = t1052 * t11237;
    (t30390, t30392, t30399, t30402, t30404, t30406)
}
