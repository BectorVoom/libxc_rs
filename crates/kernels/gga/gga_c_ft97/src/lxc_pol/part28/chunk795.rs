//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 795/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk795<F: Float>(t2210: F, t35094: F, t574: F, t5935: F, t6639: F, t1391: F, t2185: F, t6630: F, t167: F, t34817: F, t9432: F, t1017: F, t7407: F, t605: F, t1901: F, t33146: F, t35073: F, t35076: F, t35080: F, t35084: F, t35087: F, t35091: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t35095 = t2210 * t35094;
    let t35099 = t574 * t5935 * t6639;
    let t35103 = t2185 * t1391 * t6630;
    let t35107 = t9432 * t167 * t34817;
    let t35110 = t7407 * t1017;
    let t35112 = t574 * t605 * t35110;
    let t35115 = 2.0 / 3.0 * t446 * t35073 + 2.0 / 9.0 * t1901 * t35076 - 4.0 / 3.0 * t1901 * t35080 - 2.0 / 9.0 * t1901 * t35084 + 2.0 / 9.0 * t1901 * t35087 + t1901 * t35091 / 9.0 - 2.0 / 9.0 * t1901 * t35095 + t33146 + 2.0 / 3.0 * t446 * t35099 + 4.0 / 3.0 * t446 * t35103 - 2.0 * t446 * t35107 + t446 * t35112 / 3.0;
    (t35095, t35099, t35103, t35107, t35110, t35112, t35115)
}
