//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1069/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1069<F: Float>(t4840: F, t570: F, t1432: F, t1992: F, t30147: F, t7586: F, t1494: F, t7329: F, t1498: F, t30716: F, t500: F, t1181: F, t2068: F, t5080: F, t599: F) -> (F, F, F, F, F, F) {
    let t35001 = t570 * t4840;
    let t35022 = t30147 * t7586 * t1992 * t1432;
    let t35039 = t7329 * t1494;
    let t35041 = t7329 * t1498;
    let t35043 = t30716 * t500;
    let t35047 = t2068 * t1181 * t599 * t5080;
    (t35001, t35022, t35039, t35041, t35043, t35047)
}
