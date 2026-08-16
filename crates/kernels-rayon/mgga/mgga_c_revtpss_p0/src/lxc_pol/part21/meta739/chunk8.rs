//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2600/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2600(t47885: f64, t1424: f64, t1903: f64, t46362: f64, t47490: f64, t47493: f64, t47495: f64, t47497: f64, t47500: f64, t47504: f64, t47507: f64, t47510: f64, t47512: f64, t47516: f64, t47521: f64, t47863: f64, t47873: f64, t47876: f64, t9658: f64) -> f64 {
    let t47886 = 0.34697458558045176417e-2_f64 * t47885;
    let t47889 = 0.30356481678079769392e-1_f64 * t47863 + 0.9757440539382783019e-2_f64 * t47490 - 0.32927245914677557992e-1_f64 * t47493 - 0.7805952431506226415e-2_f64 * t47495 + 0.51220160311720645767e-1_f64 * t47497 + 0.39029762157531132075e-2_f64 * t47500 + t47504 - 0.29272321618148349057e-1_f64 * t47507 + 0.34697458558045176417e-2_f64 * t47510 + 0.16463622957338778996e-1_f64 * t47873 + 0.29272321618148349057e-1_f64 * t47876 + 0.15805078039045227836e2_f64 * t1424 * t46362 * t1903 * t9658 - 0.33133632253434461091e-3_f64 * t47512 - t47886 - 0.39029762157531132075e-1_f64 * t47516 - 0.69394917116090352834e-2_f64 * t47521;
    t47889
}
