//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 963/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk963<F: Float>(t21625: F, t5217: F, t5963: F, t1338: F, t1463: F, t136: F, t4046: F, t1907: F, t199: F, t203: F, t1552: F, t172: F, t674: F, t103: F, t2669: F, t2315: F, t2598: F) -> (F, F, F, F, F, F, F) {
    let t21842 = t5963 * t21625 * t5217;
    let t21991 = t1463 * t1338;
    let t22117 = 1.0 / t4046 / t136;
    let t22118 = 1.0 / t1907 / t199 * t203 * t22117;
    let t22327 = t1552 * t674 * t172;
    let t22442 = t2669 * t103;
    let t22581 = t2598 * t2315;
    (t21842, t21991, t22117, t22118, t22327, t22442, t22581)
}
