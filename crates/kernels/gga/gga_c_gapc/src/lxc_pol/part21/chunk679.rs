//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 679/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk679<F: Float>(t5: F, t825: F, t102: F, t2530: F, t1033: F, t291: F, t332: F, t327: F, t966: F, t818: F, t2404: F, t2553: F) -> (F, F, F, F, F, F, F, F) {
    let t7089 = t825 * t5;
    let t7108 = t2530 * t102;
    let t7113 = t1033 * t291;
    let t7115 = t332 * t5;
    let t7120 = t1033 * t327;
    let t7122 = t966 * t5;
    let t7158 = t966 * t818;
    let t7165 = t2553 * t2404;
    (t7089, t7108, t7113, t7115, t7120, t7122, t7158, t7165)
}
