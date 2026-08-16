//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2600/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2600<F: Float>(t47885: F, t1424: F, t1903: F, t46362: F, t47490: F, t47493: F, t47495: F, t47497: F, t47500: F, t47504: F, t47507: F, t47510: F, t47512: F, t47516: F, t47521: F, t47863: F, t47873: F, t47876: F, t9658: F) -> F {
    let t47886 = F::cast_from(0.34697458558045176417e-2_f64) * t47885;
    let t47889 = F::cast_from(0.30356481678079769392e-1_f64) * t47863 + F::cast_from(0.9757440539382783019e-2_f64) * t47490 - F::cast_from(0.32927245914677557992e-1_f64) * t47493 - F::cast_from(0.7805952431506226415e-2_f64) * t47495 + F::cast_from(0.51220160311720645767e-1_f64) * t47497 + F::cast_from(0.39029762157531132075e-2_f64) * t47500 + t47504 - F::cast_from(0.29272321618148349057e-1_f64) * t47507 + F::cast_from(0.34697458558045176417e-2_f64) * t47510 + F::cast_from(0.16463622957338778996e-1_f64) * t47873 + F::cast_from(0.29272321618148349057e-1_f64) * t47876 + F::cast_from(0.15805078039045227836e2_f64) * t1424 * t46362 * t1903 * t9658 - F::cast_from(0.33133632253434461091e-3_f64) * t47512 - t47886 - F::cast_from(0.39029762157531132075e-1_f64) * t47516 - F::cast_from(0.69394917116090352834e-2_f64) * t47521;
    t47889
}
