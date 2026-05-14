//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 522/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk522<F: Float>(t1051: F, t731: F, t763: F, t282: F, t932: F, t61: F, t126: F, t291: F) -> (F, F, F, F, F) {
    let t3182 = t731 * t1051;
    let t3184 = t763 * t1051;
    let t3186 = t932 * t282;
    let t3187 = t61 * t3186;
    let t3188 = t126 * t291;
    (t3182, t3184, t3186, t3187, t3188)
}
