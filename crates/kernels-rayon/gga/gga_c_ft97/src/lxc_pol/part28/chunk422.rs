//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 422/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk422(t27: f64, t6520: f64, t89: f64, t5673: f64, t5690: f64, t6498: f64, t6502: f64, t6506: f64, t6510: f64, t6514: f64, t6518: f64) -> (f64, f64) {
    let t6522 = t89 * t27 * t6520;
    let t6524 = t6498 / 12.0_f64 + t5673 + t6502 / 18.0_f64 + t6506 / 3.0_f64 - t6510 / 6.0_f64 + t5690 + t6514 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t6518 - t6522 / 3.0_f64;
    (t6522, t6524)
}
