//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1157/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1157<F: Float>(t33: F, t1711: F, t9617: F, t2: F, t3881: F, t1348: F, t13569: F, t22: F, t3351: F, t3842: F, t5582: F, t5585: F, t580: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t13701 = t9617 * t1711;
    let t13704 = t3881 * t2;
    let t13714 = piecewise3::<F>(t34, F::new(0.0), F::new(8.0) / F::new(27.0) * t13701 * t3842 + F::new(8.0) / F::new(9.0) * t13704 * t13569 - F::new(2.0) / F::new(9.0) * t5582 * t3351 - F::new(4.0) / F::new(3.0) * t1348 * t580 + F::new(4.0) * t5585 * t22);
    t13714
}
