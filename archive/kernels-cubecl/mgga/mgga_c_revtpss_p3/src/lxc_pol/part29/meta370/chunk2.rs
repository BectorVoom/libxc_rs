//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1328/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1328<F: Float>(t33: F, t1711: F, t9617: F, t2: F, t3881: F, t1348: F, t13569: F, t22: F, t3351: F, t3842: F, t5582: F, t5585: F, t580: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t13701 = t9617 * t1711;
    let t13704 = t3881 * t2;
    let t13714 = piecewise3::<F>(t34, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13701 * t3842 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13704 * t13569 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5582 * t3351 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1348 * t580 + F::cast_from(4.0_f64) * t5585 * t22);
    t13714
}
