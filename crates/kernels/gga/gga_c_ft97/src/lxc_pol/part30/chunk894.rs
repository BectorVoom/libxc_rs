//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 894/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk894<F: Float>(t6353: F, t7105: F, t840: F, t296: F, t36005: F, t1255: F, t2862: F, t7584: F, t34191: F, t34193: F, t34195: F, t34241: F, t36242: F, t36246: F, t36250: F, t36253: F, t36257: F, t446: F) -> (F, F, F, F) {
    let t36261 = t840 * t6353 * t7105;
    let t36264 = t296 * t36005;
    let t36268 = t2862 * t1255 * t7584;
    let t36271 = -F::new(2.0) / F::new(3.0) * t446 * t36242 + F::new(4.0) / F::new(3.0) * t446 * t36246 + F::new(2.0) / F::new(3.0) * t446 * t36250 + t34191 + t34193 - t34195 - t446 * t36253 / F::new(3.0) - t446 * t36257 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t446 * t36261 - F::new(2.0) / F::new(3.0) * t446 * t36264 + F::new(2.0) / F::new(3.0) * t446 * t36268 - t34241;
    (t36261, t36264, t36268, t36271)
}
