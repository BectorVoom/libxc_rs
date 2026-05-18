//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1104/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1104<F: Float>(t10248: F, t152673: F, t446: F, t152678: F, t43350: F, t143284: F, t3886: F, t2665: F, t24980: F, t24981: F, t28729: F, t33978: F) -> (F, F, F, F, F) {
    let t152810 = t446 * t10248 * t152673;
    let t152813 = t446 * t43350 * t152678;
    let t152815 = t143284 * t3886;
    let t152817 = t446 * t2665 * t152815;
    let t152821 = t24980 * t24981 * t33978 * t28729;
    (t152810, t152813, t152815, t152817, t152821)
}
