//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 569/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk569<F: Float>(t2574: F, t27814: F, t6119: F, t24437: F, t24447: F, t92: F, t1154: F, t747: F, t729: F, t681: F, t6899: F, t89: F, t3821: F, t6008: F, t193: F, t1131: F, t24191: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27816 = t2574 * t6119 * t27814;
    let t27817 = t24437 * t27816;
    let t27819 = t24447 * t92;
    let t27820 = t1154 * t747;
    let t27822 = t729 * t6119 * t27820;
    let t27823 = t27819 * t27822;
    let t27825 = t681 * t6899;
    let t27826 = t89 * t27825;
    let t27828 = t6008 * t3821;
    let t27829 = t193 * t27828;
    let t27830 = t89 * t27829;
    let t27832 = t24191 * t1131;
    (t27817, t27819, t27820, t27823, t27825, t27826, t27829, t27830, t27832)
}
