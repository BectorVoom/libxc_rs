//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1025/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1025<F: Float>(t12211: F, t13206: F, t1310: F, t2371: F, t10192: F, t10194: F, t10260: F, t10263: F, t10415: F, t10416: F, t10426: F, t118: F, t1315: F, t1453: F, t2320: F, t2322: F, t2328: F, t2331: F, t2372: F, t3813: F, t3821: F, t4151: F, t4254: F, t508: F, t511: F, t569: F, t649: F, t651: F, t671: F) -> (F, F, F) {
    let t13207 = t12211 + t13206;
    let t13216 = t1310 * t2371;
    let t13225 = t10192 * t511 - F::cast_from(6.0_f64) * t10194 * t508 - F::cast_from(2.0_f64) * t10260 * t651 - F::cast_from(6.0_f64) * t10263 * t651 - t10415 * t508 - F::cast_from(6.0_f64) * t10416 * t671 + t10426 * t569 - t118 * t13207 - F::cast_from(3.0_f64) * t1310 * t2320 - F::cast_from(6.0_f64) * t1310 * t2328 + F::cast_from(3.0_f64) * t1315 * t4151 - F::cast_from(6.0_f64) * t13216 * t651 + F::cast_from(3.0_f64) * t1453 * t3821 - F::cast_from(12.0_f64) * t2322 * t2331 - F::cast_from(6.0_f64) * t2322 * t2372 - F::cast_from(6.0_f64) * t2372 * t4254 - F::cast_from(3.0_f64) * t3813 * t649;
    (t13207, t13216, t13225)
}
