//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1073/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1073<F: Float>(t2405: F, t24437: F, t27762: F, t6878: F, t24438: F, t24526: F, t27775: F, t2514: F, t6135: F, t992: F, t3051: F, t6108: F, t18: F, t747: F, t108275: F, t108279: F, t108282: F, t108285: F, t108288: F, t108292: F, t108295: F, t97350: F) -> (F, F, F, F, F) {
    let t108299 = t24437 * t27762 * t6878 * t2405;
    let t108303 = t24437 * t24438 * t24526 * t27775;
    let t108308 = t24437 * t24438 * t6135 * t992 * t2514;
    let t108310 = t6108 * t3051;
    let t108314 = t108310 * t24438 * t6135 * t18 * t747;
    let t108316 = -3.0 * t108275 + t108279 + 2.0 / 9.0 * t108282 + t108285 - t97350 + 2.0 * t108288 + t108292 - t108295 / 12.0 - t108299 / 18.0 - t108303 / 6.0 - t108308 / 12.0 + t108314 / 3.0;
    (t108299, t108303, t108308, t108314, t108316)
}
