//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1001/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1001<F: Float>(t7897: F, t8995: F, t3999: F, t8085: F, t198: F, t8034: F, t2718: F, t7997: F, t670: F, t7968: F, t8107: F, t9593: F, t7983: F, t8151: F, t84: F, t8440: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109269 = t7897 * t8995;
    let t109731 = t3999 * t8085;
    let t110165 = t198 * t8034;
    let t110687 = t2718 * t7997;
    let t111018 = t7968 * t670;
    let t111176 = t8107 * t9593;
    let t111371 = t670 * t7983;
    let t111734 = t8151 * t670;
    let t119457 = t8440 * t84;
    (t109269, t109731, t110165, t110687, t111018, t111176, t111371, t111734, t119457)
}
