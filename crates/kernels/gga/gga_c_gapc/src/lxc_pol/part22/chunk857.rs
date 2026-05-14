//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 857/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk857<F: Float>(t11270: F, t8450: F, t2923: F, t5589: F, t674: F, t2906: F, t3635: F, t1736: F, t1971: F) -> (F, F, F, F) {
    let t11271 = t11270 * t8450;
    let t11273 = t2923 * t674 * t5589;
    let t11274 = t11271 * t11273;
    let t11276 = t2906 * t3635;
    let t11301 = t1736 * M_PI;
    let t11302 = t1971 * t11301;
    (t11273, t11274, t11276, t11302)
}
