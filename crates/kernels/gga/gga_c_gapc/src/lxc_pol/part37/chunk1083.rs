//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1083/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1083<F: Float>(t16181: F, t8132: F, t7591: F, t8141: F, t952: F, t291: F, t4043: F, t959: F, t1153: F, t2417: F, t6851: F, t869: F) -> (F, F, F, F, F) {
    let t16182 = t8132 * t16181;
    let t16296 = t7591 * t952 * t8141;
    let t16403 = t4043 * t291 * t959;
    let t16404 = t2417 * t1153;
    let t16408 = t869 * t6851;
    (t16182, t16296, t16403, t16404, t16408)
}
