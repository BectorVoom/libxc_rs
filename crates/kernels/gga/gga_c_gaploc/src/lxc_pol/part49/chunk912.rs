//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 912/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk912<F: Float>(t13241: F, t5552: F, t3073: F, t3263: F, t5559: F, t1960: F, t3322: F, t8440: F, t27229: F, t9777: F, t10805: F, t7324: F, t8862: F, t9780: F, t1052: F, t29646: F) -> (F, F, F, F, F, F, F, F) {
    let t44225 = 2.0 * t5552 * t13241;
    let t44228 = 6.0 * t5559 * t3073 * t3263;
    let t44231 = 2.0 * t1960 * t3073 * t3322;
    let t44232 = t8440 * t3322;
    let t44234 = 6.0 * t27229 * t9777;
    let t44236 = 4.0 * t7324 * t10805;
    let t44238 = 4.0 * t8862 * t9780;
    let t44239 = t29646 * t1052;
    (t44225, t44228, t44231, t44232, t44234, t44236, t44238, t44239)
}
