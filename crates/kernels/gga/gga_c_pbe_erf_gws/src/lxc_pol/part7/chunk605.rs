//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 605/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk605<F: Float>(t455: F, t4623: F, t1231: F, t440: F, t441: F, t1257: F, t67: F, t62: F, t1261: F, t1314: F, t457: F, t1253: F) -> (F, F, F, F, F, F, F, F) {
    let t4624 = t4623 * t455;
    let t4630 = t1231 * t440;
    let t4631 = t4630 * t441;
    let t4635 = F::new(1.0) / t1257 / t67;
    let t4636 = t62 * t4635;
    let t4637 = t4630 * t1261;
    let t4640 = t457 * t1314;
    let t4643 = t1253 * t1261;
    (t4624, t4630, t4631, t4635, t4636, t4637, t4640, t4643)
}
