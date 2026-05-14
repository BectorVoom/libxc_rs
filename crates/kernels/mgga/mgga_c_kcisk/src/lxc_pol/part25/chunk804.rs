//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 804/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk804<F: Float>(t10886: F, t5008: F, t1773: F, t1746: F, t4948: F, t1736: F, t4953: F, t4956: F, t633: F, t630: F, t45: F, t4920: F, t1704: F, t4907: F, t608: F, t4910: F, t620: F) -> (F, F, F, F, F, F, F, F) {
    let t10887 = t10886 * t5008;
    let t10888 = t1773 * t10887;
    let t10893 = t1746 * t4948;
    let t10902 = 1.0 / t4953 / t1736;
    let t10906 = 1.0 / t4956 / t633;
    let t10913 = 1.0 / t4953 / t630;
    let t10918 = t45 * t4920;
    let t10924 = 1.0 / t4907 / t1704;
    let t10925 = t608 * t10924;
    let t10928 = 1.0 / t4910 / t620;
    (t10888, t10893, t10902, t10906, t10913, t10918, t10925, t10928)
}
