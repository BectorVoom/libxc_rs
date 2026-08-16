//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 116/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk116<F: Float>(t676: F, t713: F, t27: F, t89: F, t664: F, t672: F, t661: F, t259: F, t681: F, t241: F, t683: F) -> (F, F, F, F, F) {
    let t714 = t676 * t713;
    let t716 = t89 * t27 * t714;
    let t718 = -t664 - t672 / F::cast_from(18.0_f64) - t716 / F::cast_from(6.0_f64);
    let t719 = t661 * t718;
    let t723 = t89 * t681 * t259 / F::cast_from(9.0_f64);
    let t724 = t683 * t241;
    (t714, t716, t719, t723, t724)
}
