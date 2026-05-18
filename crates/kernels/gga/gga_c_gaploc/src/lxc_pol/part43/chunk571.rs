//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 571/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk571<F: Float>(t808: F, t9688: F, t568: F, t836: F, t1445: F, t9735: F, t1457: F, t9730: F, t3266: F, t773: F, t1: F, t3209: F) -> (F, F, F, F, F, F, F) {
    let t10068 = t808 * t9688;
    let t10069 = t568 * t10068;
    let t10076 = t836 * t9688;
    let t10077 = t568 * t10076;
    let t10080 = t1445 * t9735;
    let t10083 = t1457 * t9735;
    let t10086 = t1457 * t9730;
    let t10089 = t773 * t3266;
    let t10094 = t3209 * t1;
    (t10069, t10077, t10080, t10083, t10086, t10089, t10094)
}
