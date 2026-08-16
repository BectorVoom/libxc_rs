//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 954/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk954<F: Float>(t10903: F, t3320: F, t783: F, t2078: F, t269: F, t1060: F, t10860: F, t10864: F, t10867: F, t10871: F, t10873: F, t10876: F, t10880: F, t10883: F, t10887: F, t10889: F, t10893: F, t10896: F, t10898: F, t10902: F) -> (F, F, F) {
    let t10905 = t783 * t10903 * t3320;
    let t10906 = F::cast_from(0.46574606203128791246e-1_f64) * t10905;
    let t10907 = t2078 * t269;
    let t10909 = t783 * t10907 * t1060;
    let t10911 = F::cast_from(0.43341108700271342816e-1_f64) * t10860 + t10864 + t10867 - t10871 - F::cast_from(0.86682217400542685632e-1_f64) * t10873 - F::cast_from(0.43341108700271342816e-1_f64) * t10876 - F::cast_from(0.2600466522016280569e0_f64) * t10880 - F::cast_from(0.13002332610081402845e0_f64) * t10883 + t10887 + F::cast_from(0.5200933044032561138e0_f64) * t10889 + t10893 + t10896 - t10898 - t10902 + t10906 - F::cast_from(0.21831846657716620896e-2_f64) * t10909;
    (t10905, t10907, t10911)
}
