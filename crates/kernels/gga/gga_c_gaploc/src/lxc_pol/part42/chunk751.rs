//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 751/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk751<F: Float>(t26126: F, t544: F, t18535: F, t19: F, t584: F, t60: F, t18540: F, t201: F, t1397: F, t8410: F, t1: F, t106: F, t4524: F) -> (F, F, F, F, F) {
    let t34286 = t544 * t26126;
    let t34400 = t584 * t18535 * t19 * t60;
    let t34401 = t201 * t18540;
    let t34471 = t1397 * t8410;
    let t34506 = t544 * t4524 * t1 * t106;
    (t34286, t34400, t34401, t34471, t34506)
}
