//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1130/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1130<F: Float>(t278: F, t41622: F, t123: F, t41670: F, t805: F, t41627: F, t10327: F, t1934: F, t2347: F, t274: F, t2349: F, t230: F, t2417: F) -> (F, F, F, F, F, F) {
    let t43707 = t41622 * t278;
    let t43712 = t123 / t805 / t41670;
    let t43715 = t41627 * t278;
    let t43726 = t10327 * t1934;
    let t43731 = t274 * t2347;
    let t43732 = t43731 * t2349;
    let t43736 = t230 * t2417;
    (t43707, t43712, t43715, t43726, t43732, t43736)
}
