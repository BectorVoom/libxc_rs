//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1884/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1884<F: Float>(t12211: F, t13206: F, t1310: F, t2371: F, t10192: F, t10194: F, t10260: F, t10263: F, t10415: F, t10416: F, t10426: F, t118: F, t1315: F, t1453: F, t2320: F, t2322: F, t2328: F, t2331: F, t2372: F, t3813: F, t3821: F, t4151: F, t4254: F, t508: F, t511: F, t569: F, t649: F, t651: F, t671: F) -> (F, F, F) {
    let t13207 = t12211 + t13206;
    let t13216 = t1310 * t2371;
    let t13225 = t10192 * t511 - F::new(6.0) * t10194 * t508 - F::new(2.0) * t10260 * t651 - F::new(6.0) * t10263 * t651 - t10415 * t508 - F::new(6.0) * t10416 * t671 + t10426 * t569 - t118 * t13207 - F::new(3.0) * t1310 * t2320 - F::new(6.0) * t1310 * t2328 + F::new(3.0) * t1315 * t4151 - F::new(6.0) * t13216 * t651 + F::new(3.0) * t1453 * t3821 - F::new(12.0) * t2322 * t2331 - F::new(6.0) * t2322 * t2372 - F::new(6.0) * t2372 * t4254 - F::new(3.0) * t3813 * t649;
    (t13207, t13216, t13225)
}
