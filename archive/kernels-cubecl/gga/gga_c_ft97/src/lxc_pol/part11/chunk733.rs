//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 733/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk733<F: Float>(t192: F, t9708: F, t9942: F, t2: F, t713: F, t2372: F, t2459: F, t241: F, t9567: F, t9570: F, t9571: F, t1775: F, t2503: F) -> (F, F, F, F, F, F, F, F) {
    let t9944 = t192 * t9942 * t9708;
    let t9947 = t2 * t713;
    let t9949 = t2372 * t9947 * t2459;
    let t9952 = t9567 * t241;
    let t9953 = t2 * t9570;
    let t9954 = t9953 * t9571;
    let t9955 = t9952 * t9954;
    let t9958 = t1775 * t2503;
    (t9944, t9947, t9949, t9952, t9953, t9954, t9955, t9958)
}
