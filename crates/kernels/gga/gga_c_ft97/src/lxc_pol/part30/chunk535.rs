//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 535/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk535<F: Float>(t6260: F, t668: F, t2691: F, t6248: F, t24330: F, t6242: F, t6243: F, t1701: F, sigma2: F) -> (F, F, F, F) {
    let t25044 = t6260 * t668;
    let t25049 = t2691 * t6248;
    let t25055 = t6242 * t24330 * t6243;
    let t25057 = t1701 * sigma2;
    (t25044, t25049, t25055, t25057)
}
