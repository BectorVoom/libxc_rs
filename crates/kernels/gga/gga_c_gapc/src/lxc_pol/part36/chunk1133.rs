//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1133/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1133<F: Float>(t1: F, t1736: F, t2206: F, t311: F, t3383: F, t8675: F, t1038: F, t28622: F, t4043: F, t6851: F, t1026: F, t1093: F, t2153: F) -> (F, F, F, F) {
    let t34005 = t311 * t2206 * t1736 * t1;
    let t34007 = t34005 * t8675 * t3383;
    let t34013 = t311 * t6851 * t4043 * M_PI * t1038 * t28622;
    let t34016 = t2153 * t1026 * t1093;
    (t34005, t34007, t34013, t34016)
}
