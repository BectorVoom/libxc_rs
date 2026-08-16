//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1078/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1078<F: Float>(t18427: F, t4919: F, t11547: F, t20234: F, t11546: F, t1174: F, t15265: F, t1710: F, t1717: F, t18321: F, t22035: F, t22041: F, t22047: F, t22052: F, t22056: F, t22060: F, t22063: F, t22066: F, t22069: F, t22072: F, t3447: F, t4889: F, t6120: F, t6141: F, t6147: F) -> F {
    let t22075 = t4919 * t18427;
    let t22081 = t11547 * t20234;
    let t22082 = t11546 * t22081;
    let t22085 = -F::cast_from(0.24444444444444444444e-1_f64) * t18321 * t1717 + F::cast_from(0.66666666666666666666e-2_f64) * t4889 * t6141 + F::cast_from(0.66666666666666666666e-2_f64) * t4889 * t6147 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t22035 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t22041 - F::cast_from(0.81481481481481481478e-2_f64) * t18321 * t1710 - F::cast_from(0.27777777777777777777e-3_f64) * t1174 * t22047 - F::cast_from(0.24999999999999999999e-2_f64) * t1174 * t22052 + F::cast_from(0.22222222222222222221e-2_f64) * t1174 * t22056 - F::cast_from(0.16666666666666666666e-2_f64) * t1174 * t22060 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t22063 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t22066 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t22069 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t22072 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t22075 + F::cast_from(0.14814814814814814814e-2_f64) * t15265 - F::cast_from(0.29629629629629629629e-2_f64) * t4889 * t6120 - F::cast_from(0.86419753086419753084e-3_f64) * t1174 * t22082;
    t22085
}
