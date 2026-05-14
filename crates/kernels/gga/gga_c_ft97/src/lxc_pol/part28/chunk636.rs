//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 636/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk636<F: Float>(t27066: F, t27070: F, t27075: F, t27079: F, t27084: F, t27089: F, t27094: F, t27098: F, t27101: F, t27104: F, t27107: F, t27110: F, t26768: F, t526: F, t27: F, t89: F) -> (F, F, F) {
    let t27112 = -t27066 / 3.0 - t27070 / 3.0 + t27075 / 9.0 - t27079 / 12.0 - t27084 / 12.0 + t27089 / 4.0 + t27094 / 4.0 - 2.0 / 3.0 * t27098 - 2.0 / 3.0 * t27101 + 2.0 / 9.0 * t27104 - 2.0 / 3.0 * t27107 - t27110 / 3.0;
    let t27114 = t526 * t26768;
    let t27116 = t89 * t27 * t27114;
    (t27112, t27114, t27116)
}
