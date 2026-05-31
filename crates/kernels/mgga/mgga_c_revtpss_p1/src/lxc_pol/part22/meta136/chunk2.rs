//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 910/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk910<F: Float>(t2852: F, t3252: F, t2251: F, t1012: F, t1011: F, t1017: F, t1025: F, t1028: F, t1068: F, t3188: F, t3191: F, t3194: F, t3197: F, t3203: F, t3205: F, t3208: F, t3211: F, t3216: F, t3220: F, t3224: F, t3231: F, t3234: F, t3238: F, t3241: F, t3245: F, t3248: F, t375: F) -> (F, F, F) {
    let t3253 = t3252 * t2852;
    let t3254 = t3253 * t2251;
    let t3255 = t1012 * t3254;
    let t3258 = F::cast_from(0.28582678745379824648e-3_f64) * t3188 * t1068 - F::cast_from(0.22866142996303859718e-2_f64) * t3191 * t375 + F::cast_from(0.28582678745379824648e-3_f64) * t3194 + F::cast_from(0.21437009059034868486e-3_f64) * t3197 * t375 - t3203 + F::cast_from(0.42874018118069736972e-3_f64) * t3205 * t3208 + F::cast_from(0.22866142996303859718e-2_f64) * t3211 * t1028 - F::cast_from(0.28582678745379824648e-3_f64) * t3216 - F::cast_from(0.21437009059034868486e-3_f64) * t1025 * t3220 - F::cast_from(0.42874018118069736972e-3_f64) * t3224 * t1028 + F::cast_from(0.72409452821628889107e-2_f64) * t3231 * t375 - F::cast_from(0.15244095330869239812e-2_f64) * t3234 - t1011 * t3238 / F::cast_from(144.0_f64) - t3241 * t1017 / F::cast_from(54.0_f64) + t3245 / F::cast_from(432.0_f64) + t1011 * t3248 / F::cast_from(288.0_f64) + t1011 * t3255 / F::cast_from(216.0_f64);
    (t3253, t3254, t3258)
}
