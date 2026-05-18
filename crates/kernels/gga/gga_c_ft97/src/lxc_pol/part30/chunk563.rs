//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 563/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk563<F: Float>(t25035: F, t1476: F, t2347: F, t1882: F, t6336: F, t6260: F, t668: F, t2691: F, t6248: F, t24330: F, t6242: F, t6243: F) -> (F, F, F, F, F, F) {
    let t25036 = F::new(2.0) / F::new(9.0) * t25035;
    let t25037 = t1476 * t2347;
    let t25042 = t1882 * t6336;
    let t25044 = t6260 * t668;
    let t25049 = t2691 * t6248;
    let t25055 = t6242 * t24330 * t6243;
    (t25036, t25037, t25042, t25044, t25049, t25055)
}
