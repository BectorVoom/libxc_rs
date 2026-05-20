//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1655/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1655<F: Float>(t1175: F, t12552: F, t43752: F, t439: F, t1160: F, t12408: F, t3519: F, t3522: F, t3444: F, t3451: F, t1156: F, t12428: F) -> (F, F, F, F, F, F, F, F) {
    let t45174 = t1175 * t12552;
    let t45177 = t439 * t43752;
    let t45181 = t12408 * t1160;
    let t45186 = t3519 * t3519;
    let t45187 = F::new(1.0) / t45186;
    let t45188 = t439 * t45187;
    let t45189 = t3522 * t3522;
    let t45190 = F::new(1.0) / t45189;
    let t45194 = t3444 * t3451;
    let t45197 = t1156 * t12428;
    (t45174, t45177, t45181, t45187, t45188, t45190, t45194, t45197)
}
