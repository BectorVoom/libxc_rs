//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 753/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk753<F: Float>(t1800: F, t34415: F, t1317: F, t28: F, t7172: F, t939: F, t32252: F, t32253: F, t930: F, t6437: F, t7853: F, t32147: F, t6441: F, t71: F, t938: F, t420: F) -> (F, F, F, F, F, F, F, F) {
    let t34416 = t1800 * t34415;
    let t34418 = t1317 * t28 * t34416;
    let t34421 = t7172 * t939;
    let t34424 = t32252 * t32253 * t930;
    let t34427 = t7853 * t6437;
    let t34430 = t32147 * t6441;
    let t34433 = t71 * t938;
    let t34434 = t420 * t34433;
    (t34416, t34418, t34421, t34424, t34427, t34430, t34433, t34434)
}
