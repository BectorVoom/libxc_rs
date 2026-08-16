//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1227/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1227<F: Float>(t11229: F, t2201: F, t1281: F, t27095: F, t283: F, t3424: F, t26929: F, t3177: F, t10513: F, t10691: F, t1194: F, t1095: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t92398 = t2201 * t11229;
    let t92410 = t27095 * t1281;
    let t92415 = t3424 * t283;
    let t92437 = t3177 * t26929;
    let t92447 = t10513 * t283;
    let t92486 = t10691 * t283;
    let t92514 = sigma0 * t1194;
    let t92515 = t1095 * t92514;
    (t92398, t92410, t92415, t92437, t92447, t92486, t92514, t92515)
}
