//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 875/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk875<F: Float>(t10040: F, t7720: F, t2310: F, t38351: F, t38355: F, t8571: F, t8597: F, t17859: F, t8504: F, t8508: F, t8808: F, t1971: F, t3351: F, t6558: F, t7262: F, t1939: F, t1986: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46873 = t7720 * t10040;
    let t46875 = t38351 * t2310;
    let t46877 = t38355 * t2310;
    let t46879 = t8571 * t8597;
    let t46881 = t17859 * t8504;
    let t46883 = t17859 * t8508;
    let t46885 = t17859 * t8808;
    let t46889 = t3351 * t1971 * t7262 * t6558;
    let t46891 = t1986 * t1939;
    (t46873, t46875, t46877, t46879, t46881, t46883, t46885, t46889, t46891)
}
