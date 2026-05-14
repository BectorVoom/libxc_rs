//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 984/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk984<F: Float>(t3228: F, t5047: F, t26896: F, t1021: F, t3448: F, t1096: F, t3452: F, t1196: F, t2825: F, t1200: F, t1189: F, t3178: F, t3358: F, t3355: F, t3348: F, t26889: F, t26892: F, t26894: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26897 = t5047 * t3228;
    let t26898 = t26896 * t26897;
    let t26900 = t1021 * t3448;
    let t26902 = t1096 * t3452;
    let t26904 = t2825 * t1196;
    let t26906 = t2825 * t1200;
    let t26908 = t3178 * t1189;
    let t26910 = t1021 * t3358;
    let t26912 = t1021 * t3355;
    let t26914 = t1021 * t3348;
    let t26916 = -t26889 / 64.0 + t26892 / 3.0 - t26894 / 12.0 + t26898 / 8.0 - t26900 / 96.0 + t26902 / 128.0 + t26904 / 12.0 - t26906 / 48.0 + t26908 / 64.0 + t26910 / 9.0 - 19.0 / 72.0 * t26912 - t26914 / 288.0;
    (t26897, t26898, t26900, t26902, t26904, t26906, t26908, t26910, t26912, t26914, t26916)
}
