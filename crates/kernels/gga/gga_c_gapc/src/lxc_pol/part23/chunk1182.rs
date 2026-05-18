//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1182/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1182<F: Float>(t11438: F, t26331: F, t5549: F, t1030: F, t33307: F, t4979: F, t11513: F, t1749: F, t5285: F, t1743: F, t34123: F, t11449: F, t11519: F, t1845: F, t190: F) -> (F, F, F, F, F) {
    let t34638 = t11438 * t26331 * t5549;
    let t34641 = t1030 * t33307 * t4979;
    let t34644 = t5285 * t11513 * t1749;
    let t34647 = t1743 * t34123 * t4979;
    let t34651 = t1845 * t190 * t11449 * t11519;
    (t34638, t34641, t34644, t34647, t34651)
}
