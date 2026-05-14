//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 509/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk509<F: Float>(t28: F, t6516: F, t89: F, t370: F, t6454: F, t27: F, t5673: F, t5690: F, t6498: F, t6502: F, t6506: F, t6510: F, t6514: F) -> (F, F, F, F) {
    let t6517 = t28 * t6516;
    let t6518 = t89 * t6517;
    let t6520 = t370 * t6454;
    let t6522 = t89 * t27 * t6520;
    let t6524 = t6498 / 12.0 + t5673 + t6502 / 18.0 + t6506 / 3.0 - t6510 / 6.0 + t5690 + t6514 / 9.0 + 2.0 / 3.0 * t6518 - t6522 / 3.0;
    (t6518, t6520, t6522, t6524)
}
