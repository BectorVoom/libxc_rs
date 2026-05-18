//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 735/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk735<F: Float>(t3121: F, t8636: F, t1734: F, t1903: F, t1743: F, t1912: F, t129: F, t5856: F, t197: F, t5858: F, t1878: F, t2986: F) -> (F, F, F, F, F) {
    let t8637 = t3121 * t8636;
    let t8639 = t1734 * t1903;
    let t8641 = t1743 * t8639 * t1912;
    let t8643 = t5856 * t129;
    let t8644 = t197 * t5858;
    let t8645 = t8643 * t8644;
    let t8647 = t2986 * t1878;
    (t8637, t8639, t8641, t8645, t8647)
}
