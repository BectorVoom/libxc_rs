//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 744/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk744<F: Float>(t33: F, t516: F, t1113: F, t3881: F, t1348: F, t3351: F, t9351: F, t9357: F, t9614: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t9615 = t33 * t33;
    let t9617 = F::new(1.0) / t516 / t9615;
    let t9620 = t3881 * t1113;
    let t9626 = piecewise3::<F>(t34, F::new(0.0), F::new(8.0) / F::new(27.0) * t9617 * t9351 - F::new(2.0) / F::new(3.0) * t9620 * t3351 + F::new(2.0) / F::new(3.0) * t1348 * t9357);
    let t9628 = t9614 / F::new(2.0) + t9626 / F::new(2.0);
    t9628
}
