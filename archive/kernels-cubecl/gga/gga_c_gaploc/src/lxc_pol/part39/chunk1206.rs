//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1206/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1206<F: Float>(t2299: F, t3689: F, t1415: F, t1646: F, t1: F, t544: F, t594: F, t2392: F, t47953: F, t6710: F, t6711: F, t12092: F, t2478: F, t6583: F) -> (F, F, F, F) {
    let t48165 = t2299 * t3689;
    let t48167 = t1415 * t48165 * t1646;
    let t48171 = t544 * t594 * t3689 * t1;
    let t48172 = t48171 * t2392;
    let t48175 = t6710 * t6711 * t47953;
    let t48178 = t6583 * t12092 * t2478;
    (t48167, t48172, t48175, t48178)
}
