//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 338/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk338<F: Float>(t6008: F, t6009: F, t193: F, t1410: F, t25: F, t3762: F, t694: F) -> (F, F, F, F, F) {
    let t6010 = t6008 * t6009;
    let t6011 = t193 * t6010;
    let t6014 = t1410 * t25;
    let t6015 = t6014 * t3762;
    let t6018 = t694 * t1410;
    (t6010, t6011, t6014, t6015, t6018)
}
