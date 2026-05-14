//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1150/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1150<F: Float>(t25885: F, t93506: F, t23054: F, t25872: F, t1808: F, t1871: F, t22952: F, t5675: F, t942: F, t23050: F, t25899: F, t2: F, t25846: F, t1564: F, t379: F, t5674: F) -> (F, F, F, F, F, F, F) {
    let t100270 = t93506 * t25885;
    let t100271 = t100270 / 9.0;
    let t100272 = t23054 * t25872;
    let t100273 = 2.0 / 3.0 * t100272;
    let t100277 = t22952 * t1871 * t5675 * t942 * t1808;
    let t100283 = t22952 * t1871 * t25899 * t23050;
    let t100285 = t2 * t25846;
    let t100288 = t5674 * t1564 * t100285 * t379;
    (t100270, t100271, t100272, t100273, t100277, t100283, t100288)
}
