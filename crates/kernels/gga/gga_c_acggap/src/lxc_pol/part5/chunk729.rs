//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 729/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk729<F: Float>(t1775: F, t330: F, t1165: F, t1889: F, t407: F, t1894: F, t1181: F, t1899: F, t1439: F, t4643: F, t372: F, t960: F, t1323: F, t4593: F, t1327: F, t1314: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5884 = t330 * t1775;
    let t5891 = t1165 * t1889 * t407;
    let t5894 = t1894 * t407;
    let t5895 = t1181 * t5894;
    let t5899 = t1165 * t1899 * t407;
    let t5902 = t4643 * t1439;
    let t5903 = t1181 * t5902;
    let t5906 = t1889 * t372;
    let t5907 = t960 * t5906;
    let t5910 = t4593 * t1323;
    let t5913 = t4593 * t1327;
    let t5916 = t4593 * t1314;
    (t5884, t5891, t5894, t5895, t5899, t5902, t5903, t5906, t5907, t5910, t5913, t5916)
}
