//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1411/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1411<F: Float>(t35422: F, t35451: F, t36346: F, t36347: F, t36349: F, t36350: F, t36351: F, t36352: F, t36353: F, t36354: F, t36355: F, t36357: F, t36358: F) -> F {
    let t38542 = t36346 + t36347 - F::new(0.5431140175846100239e-5) * t35422 + t36349 - t36350 - t36351 - t36352 - t36353 - t36354 + t36355 - F::new(0.49106559089941822994e-4) * t35451 + t36357 + t36358;
    t38542
}
