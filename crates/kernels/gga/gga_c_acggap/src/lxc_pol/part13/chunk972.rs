//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 972/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk972<F: Float>(t1494: F, t7329: F, t1498: F, t30716: F, t500: F, t1181: F, t2068: F, t5080: F, t599: F, t1411: F, t1983: F, t7585: F, t7586: F, t1165: F, t4555: F, t604: F, t7493: F) -> (F, F, F, F, F, F) {
    let t35039 = t7329 * t1494;
    let t35040 = 7.0 / 72.0 * t35039;
    let t35041 = t7329 * t1498;
    let t35042 = 7.0 / 72.0 * t35041;
    let t35043 = t30716 * t500;
    let t35047 = t2068 * t1181 * t599 * t5080;
    let t35051 = t7585 * t7586 * t1983 * t1411;
    let t35052 = 0.14291339372689912324e-3 * t35051;
    let t35055 = t7493 * t1165 * t604 * t4555;
    (t35040, t35042, t35043, t35047, t35052, t35055)
}
