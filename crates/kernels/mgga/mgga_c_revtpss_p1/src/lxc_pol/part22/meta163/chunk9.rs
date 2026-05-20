//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1098/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1098<F: Float>(t33: F, t1348: F, t3351: F, t3842: F, t3881: F, t3880: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t3887 = piecewise3::<F>(t34, F::new(0.0), -F::new(2.0) / F::new(9.0) * t3881 * t3842 + F::new(2.0) / F::new(3.0) * t1348 * t3351);
    let t3889 = t3880 / F::new(2.0) + t3887 / F::new(2.0);
    t3889
}
