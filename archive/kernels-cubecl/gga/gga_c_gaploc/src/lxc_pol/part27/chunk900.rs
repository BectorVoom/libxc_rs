//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 900/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk900<F: Float>(t6509: F, t883: F, t4782: F, t9272: F, t123: F, t2293: F, t912: F, t587: F, t544: F, t6603: F) -> (F, F, F, F, F, F) {
    let t9273 = t883 * t6509;
    let t9274 = t4782 * t9273;
    let t9276 = F::cast_from(0.11502877786176224903e1_f64) * t9272 * t9274;
    let t9277 = t2293 * t123;
    let t9278 = t9277 * t883;
    let t9279 = t912 * t9278;
    let t9280 = t587 * t9279;
    let t9285 = t544 * t6603;
    (t9274, t9276, t9278, t9279, t9280, t9285)
}
