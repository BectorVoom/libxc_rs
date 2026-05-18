//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 790/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk790<F: Float>(t3034: F, t9223: F, t3100: F, t659: F, t2979: F, t5856: F, t1504: F, t2982: F, t1875: F, t9128: F, t2983: F, t1027: F, t1781: F) -> (F, F, F, F, F, F) {
    let t9224 = t3034 * t9223;
    let t9226 = t3100 * t659;
    let t9228 = t5856 * t2979;
    let t9229 = t2982 * t1504;
    let t9230 = t9228 * t9229;
    let t9232 = t1875 * t9128;
    let t9233 = t9232 * t2983;
    let t9235 = t1027 * t1781;
    (t9224, t9226, t9229, t9230, t9233, t9235)
}
