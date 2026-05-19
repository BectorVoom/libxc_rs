//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1291/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1291<F: Float>(t11233: F, t18589: F, t18592: F, t851: F, t11286: F, t2281: F, t10006: F, t10016: F, t10019: F, t10020: F, t11167: F, t11269: F, t11287: F, t18706: F, t2257: F, t22767: F, t2279: F, t22829: F, t3102: F, t31394: F, t31397: F, t31400: F, t31404: F, t31407: F, t3796: F, t6288: F, t6313: F, t8120: F, t870: F) -> (F, F) {
    let t31411 = F::cast_from(0.24955700379505800916e5_f64) * t18589 * t11233 * t18592 * t851;
    let t31430 = t11286 * t2281;
    let t31437 = t31394 + t31397 + t31400 - t31404 - t31407 - t31411 - F::cast_from(0.57895126195293126241e3_f64) * t22829 * t10006 + F::cast_from(0.1929837539843104208e3_f64) * t8120 * t10016 + F::cast_from(0.62071215503128080361e4_f64) * t22767 * t10020 + F::cast_from(0.11579025239058625248e4_f64) * t6288 * t11269 * t870 - F::cast_from(0.57895126195293126243e3_f64) * t6313 * t3796 * t3102 - F::cast_from(0.24828486201251232145e5_f64) * t18706 * t11167 * t870 - F::new(2.0) * t2257 * t11287 * t870 + F::cast_from(0.32163958997385070134e2_f64) * t2279 * t31430 * t870 + F::cast_from(0.6207121550312808036e4_f64) * t6288 * t10019 * t3102;
    (t31411, t31437)
}
