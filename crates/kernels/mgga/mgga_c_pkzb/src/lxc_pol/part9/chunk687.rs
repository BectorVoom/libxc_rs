//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 687/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk687<F: Float>(t4793: F, t1425: F, t440: F, t1424: F, t1431: F, t8: F, t82: F) -> (F, F, F, F, F) {
    let t4794 = 1.0 / t4793;
    let t4795 = t1425 * t440;
    let t4796 = t4794 * t4795;
    let t4799 = t1424 * t440;
    let t4800 = t4799 * t1431;
    let t4803 = t8 * t82;
    (t4794, t4795, t4796, t4800, t4803)
}
