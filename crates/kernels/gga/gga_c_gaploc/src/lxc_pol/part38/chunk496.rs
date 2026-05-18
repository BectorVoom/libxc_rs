//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 496/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk496<F: Float>(t2482: F, t2492: F, t9267: F, t4779: F, t6574: F, t584: F) -> (F, F, F) {
    let t9268 = t2492 * t2482;
    let t9270 = F::new(0.19171462976960374838e1) * t9267 * t9268;
    let t9271 = t4779 * t6574;
    let t9272 = t584 * t9271;
    (t9270, t9271, t9272)
}
