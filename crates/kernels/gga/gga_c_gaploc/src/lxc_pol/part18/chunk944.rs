//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 944/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk944<F: Float>(t3073: F, t977: F, t1960: F, t2595: F, t8862: F, t2592: F, t10282: F, t10285: F, t10288: F, t10289: F, t10297: F, t10300: F, t10625: F, t11125: F, t11127: F, t11130: F, t11132: F, t11134: F, t748: F) -> (F, F) {
    let t11135 = t3073 * t977;
    let t11137 = 2.0 * t1960 * t11135;
    let t11139 = 2.0 * t8862 * t2595;
    let t11140 = t2592 * t3073;
    let t11141 = -t11125 * t748 + 2.0 * t11127 * t1960 - t10282 + t10285 + t10288 + t10289 + t10297 - t10300 + t10625 - t11130 + t11132 + t11134 + t11137 + t11139 - t11140;
    (t11135, t11141)
}
