//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1026/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1026<F: Float>(t1383: F, t30273: F, t1186: F, t30294: F, t1398: F, t14083: F, t14084: F, t165: F, t173: F, t20781: F, t20783: F, t25538: F, t25540: F, t25542: F, t25544: F, t30852: F, t3891: F) -> F {
    let t30858 = t1383 * t30273;
    let t30861 = t1186 * t30294;
    let t30864 = t1398 * t30273;
    let t30873 = F::cast_from(0.46615850170166761884e-3_f64) * t3891 * t30852 - t14083 + t14084 - F::cast_from(0.4755e-2_f64) * t165 * t30858 - F::cast_from(0.1585e-2_f64) * t165 * t30861 - F::cast_from(0.30247875e-4_f64) * t173 * t30864 - F::cast_from(0.35867157975189532869e-1_f64) * t25538 + F::cast_from(0.31077233446777841256e-3_f64) * t25540 + F::cast_from(0.71734315950379065738e-1_f64) * t25542 - F::cast_from(0.93231700340333523768e-3_f64) * t25544 + F::cast_from(0.71734315950379065738e-1_f64) * t20781 - F::cast_from(0.93231700340333523768e-3_f64) * t20783;
    t30873
}
