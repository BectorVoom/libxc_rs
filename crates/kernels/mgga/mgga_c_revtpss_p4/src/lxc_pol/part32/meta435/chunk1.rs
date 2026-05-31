//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1563/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1563<F: Float>(t21258: F, t3720: F, t1222: F, t1266: F, t17629: F, t21228: F, t21234: F, t21236: F, t21239: F, t21242: F, t21246: F, t21249: F, t21252: F, t21255: F, t3625: F, t3718: F, t5381: F, t5384: F, t5397: F) -> F {
    let t21259 = t3720 * t21258;
    let t21264 = -F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t21228 + t17629 / F::cast_from(648.0_f64) + F::cast_from(0.15879265969655458138e-3_f64) * t21234 + t1222 * t21236 / F::cast_from(108.0_f64) + t1222 * t21239 / F::cast_from(36.0_f64) + F::cast_from(0.15244095330869239812e-2_f64) * t21242 * t1266 + F::cast_from(0.42874018118069736972e-3_f64) * t5384 * t21246 + t21249 / F::cast_from(162.0_f64) - t21252 / F::cast_from(864.0_f64) - t21255 / F::cast_from(432.0_f64) - F::cast_from(0.42874018118069736972e-3_f64) * t3718 * t21259 - F::cast_from(0.28582678745379824648e-3_f64) * t5381 * t5397;
    t21264
}
