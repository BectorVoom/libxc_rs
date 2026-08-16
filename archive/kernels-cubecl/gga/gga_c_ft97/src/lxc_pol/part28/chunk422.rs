//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 422/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk422<F: Float>(t27: F, t6520: F, t89: F, t5673: F, t5690: F, t6498: F, t6502: F, t6506: F, t6510: F, t6514: F, t6518: F) -> (F, F) {
    let t6522 = t89 * t27 * t6520;
    let t6524 = t6498 / F::cast_from(12.0_f64) + t5673 + t6502 / F::cast_from(18.0_f64) + t6506 / F::cast_from(3.0_f64) - t6510 / F::cast_from(6.0_f64) + t5690 + t6514 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6518 - t6522 / F::cast_from(3.0_f64);
    (t6522, t6524)
}
