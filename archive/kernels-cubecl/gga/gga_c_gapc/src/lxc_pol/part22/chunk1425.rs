//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1425/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1425<F: Float>(t12042: F, t12151: F, t36091: F, t36092: F, t36093: F, t37313: F, t37314: F, t37317: F, t37318: F, t37319: F, t37320: F, t37322: F, t37323: F, t37324: F, t37325: F, t37326: F, t37327: F, t37328: F, t37329: F, t7: F) -> F {
    let t37330 = F::cast_from(2.0_f64) * t12042;
    let tv4rho2sigma21 = -t36091 - t36092 + t36093 + t7 * (t37313 + t37314) + t37317 - t37318 + t37319 - t37320 + F::cast_from(2.0_f64) * t12151 + t37322 - t37323 + t37324 + t37325 - t37326 - t37327 + t37328 + t37329 - t37330;
    tv4rho2sigma21
}
