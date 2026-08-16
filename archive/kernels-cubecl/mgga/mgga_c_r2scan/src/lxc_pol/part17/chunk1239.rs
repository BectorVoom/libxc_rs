//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1239/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1239<F: Float>(t43528: F, t43531: F, t43533: F, t43536: F, t43539: F, t43541: F, t43544: F, t43547: F, t43549: F, t43551: F, t43553: F, t43555: F) -> F {
    let t44440 = -F::cast_from(0.87327386630866483588e-2_f64) * t43528 - F::cast_from(0.87327386630866483588e-2_f64) * t43531 + F::cast_from(0.87327386630866483588e-2_f64) * t43533 - F::cast_from(0.43663693315433241794e-2_f64) * t43536 - F::cast_from(0.13099107994629972538e-1_f64) * t43539 - F::cast_from(0.26198215989259945076e-1_f64) * t43541 - F::cast_from(0.26198215989259945076e-1_f64) * t43544 - F::cast_from(0.26198215989259945076e-1_f64) * t43547 - F::cast_from(0.86682217400542685632e-1_f64) * t43549 - F::cast_from(0.87327386630866483588e-2_f64) * t43551 - F::cast_from(0.47609969197673950973e-2_f64) * t43553 - F::cast_from(0.2600466522016280569e0_f64) * t43555;
    t44440
}
