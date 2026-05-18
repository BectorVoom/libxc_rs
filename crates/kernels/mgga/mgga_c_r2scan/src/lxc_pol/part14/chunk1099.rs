//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1099/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1099<F: Float>(t11310: F, t1338: F, t3552: F, t6755: F, t1142: F, t19309: F, t1348: F, t6767: F, t19327: F, t1114: F, t23040: F, t3493: F, t481: F) -> (F, F, F, F, F, F, F, F) {
    let t38953 = t1338 * t11310;
    let t38958 = t6755 * t3552;
    let t38961 = t19309 * t1142;
    let t38966 = t1348 * t11310;
    let t38971 = t6767 * t3552;
    let t38976 = t19327 * t1142;
    let t39010 = t23040 * t1114;
    let t39014 = t3493 * t481;
    (t38953, t38958, t38961, t38966, t38971, t38976, t39010, t39014)
}
