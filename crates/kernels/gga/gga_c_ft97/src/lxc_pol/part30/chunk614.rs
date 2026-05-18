//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 614/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk614<F: Float>(t27742: F, t675: F, t263: F, t193: F, t13927: F, t6175: F, t24412: F, t3864: F, t681: F, t6843: F, t1168: F, t6187: F) -> (F, F, F, F, F, F) {
    let t27906 = t675 * t27742;
    let t27907 = t27906 * t263;
    let t27908 = t193 * t27907;
    let t27911 = t13927 * t6175;
    let t27913 = t24412 * t3864;
    let t27915 = t681 * t6843;
    let t27924 = t6187 * t1168;
    (t27906, t27908, t27911, t27913, t27915, t27924)
}
