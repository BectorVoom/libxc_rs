//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 850/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk850<F: Float>(t10401: F, t617: F, t7499: F, t5211: F, t10402: F, t7759: F, t7115: F, t2666: F, t7106: F, t1022: F, t7483: F, t2673: F, t10417: F, t10421: F, t10423: F, t10428: F, t10432: F, t10436: F, t10441: F, t10446: F, t10450: F, t10454: F, t5205: F, t7190: F, t7193: F) -> (F, F, F, F, F) {
    let t10456 = t7499 * t10401 * t617;
    let t10458 = 16.0 / 45.0 * t5211 * t10456;
    let t10459 = t7759 * t10402;
    let t10461 = 8.0 / 27.0 * t7115 * t10459;
    let t10462 = t7106 * t2666;
    let t10464 = 16.0 / 45.0 * t5211 * t10462;
    let t10465 = t7483 * t1022;
    let t10466 = t10465 * t2673;
    let t10468 = 32.0 / 45.0 * t5211 * t10466;
    let t10469 = t10417 + t10421 + 2.0 / 135.0 * t5205 + t10423 - t7190 + t7193 + t10428 + t10432 - t10436 - t10441 + t10446 + t10450 + t10454 - t10458 - t10461 - t10464 - t10468;
    (t10458, t10461, t10464, t10468, t10469)
}
