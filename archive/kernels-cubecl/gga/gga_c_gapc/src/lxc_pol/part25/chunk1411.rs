//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1411/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1411<F: Float>(t35482: F, t36361: F, t36362: F, t36363: F, t36364: F, t36365: F, t36366: F, t36368: F, t36369: F, t36370: F, t36371: F, t36372: F, t36373: F) -> F {
    let t38545 = -t36361 - t36362 - t36363 + t36364 + t36365 - t36366 + F::cast_from(0.42242201367691890747e-5_f64) * t35482 - t36368 - t36369 + t36370 + t36371 - t36372 + t36373;
    t38545
}
