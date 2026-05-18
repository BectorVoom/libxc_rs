//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 711/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk711<F: Float>(t676: F, t9692: F, t27: F, t89: F, t10: F, t242: F, t3050: F, t1636: F, t714: F, t669: F, t8608: F, t666: F) -> (F, F, F, F, F, F, F) {
    let t9693 = t676 * t9692;
    let t9695 = t89 * t27 * t9693;
    let t9698 = t10 * t3050 * t242;
    let t9699 = F::new(14.0) / F::new(81.0) * t9698;
    let t9701 = t89 * t1636 * t714;
    let t9703 = t669 * t8608;
    let t9705 = t89 * t666 * t9703;
    (t9693, t9695, t9698, t9699, t9701, t9703, t9705)
}
