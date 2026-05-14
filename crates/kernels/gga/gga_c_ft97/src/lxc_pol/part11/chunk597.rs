//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 597/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk597<F: Float>(t8659: F, t8721: F, t184: F, t21: F, t2304: F, t648: F, t2299: F, t3664: F, t1580: F, t649: F, t2300: F, t363: F, t2305: F, t2252: F, t342: F, t511: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8722 = t8659 + t8721;
    let t8723 = t8722 * t184;
    let t8724 = t8723 * t21;
    let t8730 = t2304 * t648;
    let t8731 = t8730 * t184;
    let t8732 = t8731 * t21;
    let t8738 = t2299 * t648;
    let t8739 = t8738 * t3664;
    let t8744 = t649 * t1580;
    let t8751 = t2300 * t363;
    let t8754 = t2305 * t363;
    let t8759 = t342 * t2252 * t511 / 18.0;
    (t8722, t8723, t8724, t8731, t8732, t8738, t8739, t8744, t8751, t8754, t8759)
}
