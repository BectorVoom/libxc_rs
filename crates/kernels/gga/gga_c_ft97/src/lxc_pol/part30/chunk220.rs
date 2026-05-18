//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 220/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk220<F: Float>(t2336: F, t670: F, t89: F, t375: F, t714: F, t190: F, t322: F) -> (F, F, F, F, F) {
    let t2338 = t89 * t2336 * t670;
    let t2339 = t2338 / F::new(27.0);
    let t2341 = t89 * t375 * t714;
    let t2342 = t2341 / F::new(9.0);
    let t2344 = F::new(1.0) / t322 / t190;
    (t2338, t2339, t2341, t2342, t2344)
}
