//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1096/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1096<F: Float>(t21625: F, t5217: F, t5963: F, t1338: F, t1463: F, t136: F, t4046: F, t1907: F, t199: F, t203: F, t1552: F, t172: F, t674: F) -> (F, F, F, F, F) {
    let t21842 = t5963 * t21625 * t5217;
    let t21991 = t1463 * t1338;
    let t22117 = F::new(1.0) / t4046 / t136;
    let t22118 = F::new(1.0) / t1907 / t199 * t203 * t22117;
    let t22327 = t1552 * t674 * t172;
    (t21842, t21991, t22117, t22118, t22327)
}
