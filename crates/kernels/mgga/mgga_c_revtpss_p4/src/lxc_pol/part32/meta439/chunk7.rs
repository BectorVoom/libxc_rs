//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1602/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1602<F: Float>(t33: F, t3881: F, t6416: F, t1113: F, t1348: F, t20256: F, t21956: F, t2255: F, t5582: F, t21955: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t21961 = t3881 * t6416;
    let t21967 = piecewise3::<F>(t34, F::new(0.0), F::new(8.0) / F::new(27.0) * t21956 * t1113 + F::new(8.0) / F::new(9.0) * t5582 * t2255 - F::new(2.0) / F::new(9.0) * t21961 * t1113 + F::new(2.0) / F::new(3.0) * t1348 * t20256);
    let t21969 = t21955 / F::new(2.0) + t21967 / F::new(2.0);
    t21969
}
