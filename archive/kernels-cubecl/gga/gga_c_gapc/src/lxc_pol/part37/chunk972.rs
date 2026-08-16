//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 972/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk972<F: Float>(t11303: F, t5218: F, t5967: F, t1673: F, t3713: F, t3709: F, t126: F, t195: F) -> (F, F, F, F, F) {
    let t11304 = t11303 * t5218;
    let t11306 = t11303 * t5967;
    let t11308 = t1673 * t3713;
    let t11309 = t3709 * t11308;
    let t11311 = t126 * t195;
    (t11304, t11306, t11308, t11309, t11311)
}
