//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 793/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk793<F: Float>(t200: F, t9078: F, t3000: F, t2996: F, t126: F, t1636: F, t1875: F, t4940: F, t8769: F, t5190: F, t116: F, t5294: F) -> (F, F, F, F, F, F, F) {
    let t9079 = t9078 * t200;
    let t9080 = t9079 * t3000;
    let t9081 = t2996 * t9080;
    let t9083 = t126 * t1636;
    let t9084 = t1875 * t9083;
    let t9085 = t9084 * t4940;
    let t9087 = t1875 * t8769;
    let t9088 = t9087 * t5190;
    let t9090 = t116 * t5294;
    (t9079, t9080, t9081, t9083, t9085, t9088, t9090)
}
