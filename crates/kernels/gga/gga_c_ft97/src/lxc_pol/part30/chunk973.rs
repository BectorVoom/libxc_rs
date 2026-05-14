//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 973/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk973<F: Float>(t143163: F, t152799: F, t33820: F, t35981: F, t681: F, t89: F, t35993: F, t25162: F, t35860: F, t10683: F, t28501: F, t6317: F, t6318: F, t1091: F, t143144: F, t2665: F) -> (F, F, F, F, F, F) {
    let t152899 = t33820 * t143163 * t152799;
    let t152902 = t89 * t681 * t35981;
    let t152905 = t89 * t681 * t35993;
    let t152907 = t25162 * t35860;
    let t152913 = t6317 * t10683 * t6318 * t28501;
    let t152917 = t6317 * t2665 * t143144 * t1091;
    (t152899, t152902, t152905, t152907, t152913, t152917)
}
