//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 605/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk605<F: Float>(t27805: F, t27807: F, t375: F, t6903: F, t89: F, t1131: F, t747: F, t2574: F, t6119: F, t24437: F, t24447: F, t92: F) -> (F, F, F, F, F) {
    let t27808 = t27805 * t27807;
    let t27811 = t89 * t375 * t6903;
    let t27814 = t1131 * t747;
    let t27816 = t2574 * t6119 * t27814;
    let t27817 = t24437 * t27816;
    let t27819 = t24447 * t92;
    (t27808, t27811, t27814, t27817, t27819)
}
