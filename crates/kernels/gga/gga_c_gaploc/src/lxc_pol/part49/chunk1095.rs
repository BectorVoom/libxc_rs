//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1095/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1095<F: Float>(t47067: F, t47072: F, t42498: F, t12032: F, t2497: F, t12148: F, t1382: F, t921: F, t13838: F, t5559: F, t841: F, t12270: F, t1960: F, t977: F) -> (F, F, F, F, F, F) {
    let t47073 = t47067 + t47072;
    let t47074 = F::new(6.0) * t42498;
    let t47075 = t12032 * t2497;
    let t47077 = t1382 * t12148 * t921;
    let t47078 = F::new(2.0) * t47077;
    let t47080 = t5559 * t13838 * t841;
    let t47083 = t1960 * t12270 * t977;
    (t47073, t47074, t47075, t47078, t47080, t47083)
}
