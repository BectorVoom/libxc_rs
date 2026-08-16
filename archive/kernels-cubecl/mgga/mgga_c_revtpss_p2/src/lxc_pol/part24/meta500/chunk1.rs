//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1504/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1504<F: Float>(t23359: F, t686: F, t72: F, t874: F, t10871: F, t6016: F, t4500: F, t62808: F, t125: F, t23148: F, t23167: F, t23244: F) -> (F, F, F, F, F, F) {
    let t76237 = t874 * t23359 * t72 * t686;
    let t76242 = t10871 * t6016;
    let t76255 = t62808 * t4500;
    let t76279 = t125 * t23148;
    let t76284 = t125 * t23167;
    let t76289 = t125 * t23244;
    (t76237, t76242, t76255, t76279, t76284, t76289)
}
