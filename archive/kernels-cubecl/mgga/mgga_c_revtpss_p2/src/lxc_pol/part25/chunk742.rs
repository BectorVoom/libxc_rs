//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 742/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk742<F: Float>(t1017: F, t1028: F, t1047: F, t1068: F, t348: F, t375: F, t7106: F, t7110: F, t7111: F, t7114: F, t7117: F, t7122: F, t7126: F, t7130: F, t7132: F) -> F {
    let t7135 = -t7106 * t348 / F::cast_from(36.0_f64) + t7110 + t7111 * t1017 / F::cast_from(288.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t7114 * t375 - F::cast_from(0.42874018118069736972e-3_f64) * t7117 * t1028 + F::cast_from(0.42874018118069736972e-3_f64) * t7122 * t1047 - F::cast_from(0.22866142996303859718e-2_f64) * t7126 * t375 + t7130 + F::cast_from(0.28582678745379824648e-3_f64) * t7132 * t1068;
    t7135
}
