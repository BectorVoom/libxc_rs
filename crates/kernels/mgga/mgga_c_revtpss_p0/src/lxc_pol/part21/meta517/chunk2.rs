//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2152/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2152<F: Float>(t1043: F, t905: F, t606: F, t3155: F, t15691: F, t1047: F, t1063: F, t11656: F, t11977: F, t15700: F, t16190: F, t16196: F, t16201: F, t16205: F, t16210: F, t16218: F, t16220: F, t16223: F, t16226: F, t1671: F, t3169: F, t4825: F, t4869: F) -> (F, F, F, F) {
    let t16227 = t1043 * t905;
    let t16228 = t16227 * t606;
    let t16229 = t3155 * t16228;
    let t16230 = t15691 * t16229;
    let t16233 = -F::cast_from(0.22866142996303859718e-2_f64) * t16190 * t1047 + F::cast_from(0.15244095330869239812e-2_f64) * t11656 * t4825 - F::cast_from(0.28582678745379824648e-3_f64) * t1063 * t16196 - F::cast_from(0.14291339372689912324e-2_f64) * t1063 * t16201 + F::cast_from(0.23818898954483187207e-3_f64) * t1063 * t16205 + F::cast_from(0.63517063878621832552e-3_f64) * t1063 * t16210 - F::cast_from(0.22866142996303859718e-2_f64) * t11977 * t1671 - F::cast_from(0.22866142996303859718e-2_f64) * t3169 * t4869 + t16218 - t16220 / F::new(1296.0) + F::cast_from(0.47637797908966374414e-3_f64) * t15700 * t16223 + F::cast_from(0.57165357490759649296e-3_f64) * t16226 * t16230;
    (t16227, t16229, t16230, t16233)
}
