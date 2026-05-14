//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1194/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1194<F: Float>(t35422: F, t35451: F, t36346: F, t36347: F, t36349: F, t36350: F, t36351: F, t36352: F, t36353: F, t36354: F, t36355: F, t36357: F, t36358: F, t35482: F, t36361: F, t36362: F, t36363: F, t36364: F, t36365: F, t36366: F, t36368: F, t36369: F, t36370: F, t36371: F, t36372: F, t36373: F) -> (F, F) {
    let t38542 = t36346 + t36347 - 0.5431140175846100239e-5 * t35422 + t36349 - t36350 - t36351 - t36352 - t36353 - t36354 + t36355 - 0.49106559089941822994e-4 * t35451 + t36357 + t36358;
    let t38545 = -t36361 - t36362 - t36363 + t36364 + t36365 - t36366 + 0.42242201367691890747e-5 * t35482 - t36368 - t36369 + t36370 + t36371 - t36372 + t36373;
    (t38542, t38545)
}
