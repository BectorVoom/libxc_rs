//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3246/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3246<F: Float>(t33: F, t1113: F, t6416: F, t580: F, t1348: F, t13701: F, t13704: F, t20256: F, t21956: F, t2255: F, t22778: F, t22783: F, t3881: F, t46328: F, t5582: F, t81123: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t85426 = t6416 * t1113;
    let t85429 = t580 * t6416;
    let t85440 = piecewise3::<F>(t34, F::new(0.0), -F::new(56.0) / F::new(81.0) * t46328 * t22778 * t1113 - F::new(16.0) / F::new(9.0) * t21956 * t2255 + F::new(8.0) / F::new(9.0) * t13701 * t85426 + F::new(4.0) / F::new(3.0) * t13704 * t85429 - F::new(2.0) / F::new(3.0) * t5582 * t20256 - F::new(2.0) / F::new(9.0) * t3881 * t22783 * t1113 + F::new(2.0) / F::new(3.0) * t1348 * t81123);
    (t85426, t85429, t85440)
}
