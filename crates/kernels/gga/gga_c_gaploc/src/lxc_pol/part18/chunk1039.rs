//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1039/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1039<F: Float>(t20692: F, t2365: F, t7025: F, t4130: F, t874: F, t6907: F, t9272: F, t1265: F, t587: F, t9438: F, t9439: F, t2389: F, t6700: F, t6696: F, t1441: F, t9264: F) -> (F, F, F, F, F, F) {
    let t31178 = 0.59584149919750711116e-1 * t7025 * t2365 * t20692;
    let t31187 = t4130 * t874;
    let t31190 = 0.10352590007558602413e2 * t9272 * t31187 * t6907;
    let t31207 = t587 * t9438 * t9439 * t1265;
    let t31213 = 0.11916829983950142223e0 * t6700 * t2389;
    let t31215 = 0.11916829983950142223e0 * t6696 * t2389;
    let t31216 = t1441 * t9264;
    (t31178, t31190, t31207, t31213, t31215, t31216)
}
