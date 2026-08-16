//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 599/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk599<F: Float>(t27742: F, t743: F, t1434: F, t193: F, t10157: F, t3837: F, t6119: F, t6118: F, t3875: F, t6135: F, t24432: F, t24531: F, t3886: F) -> (F, F, F, F, F) {
    let t27743 = t743 * t27742;
    let t27745 = t1434 * t193 * t27743;
    let t27750 = t10157 * t6119 * t3837;
    let t27751 = t6118 * t27750;
    let t27753 = t6135 * t3875;
    let t27754 = t24432 * t27753;
    let t27755 = t6118 * t27754;
    let t27757 = t24531 * t3886;
    (t27745, t27751, t27753, t27755, t27757)
}
