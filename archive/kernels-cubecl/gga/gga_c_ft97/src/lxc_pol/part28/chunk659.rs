//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 659/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk659<F: Float>(t26183: F, t26228: F, t26275: F, t26315: F, t26363: F, t26407: F, t26457: F, t26490: F, t1058: F, t5843: F, t28: F, t609: F, t6718: F) -> (F, F, F) {
    let t26493 = t26183 + t26228 + t26275 + t26315 + t26363 + t26407 + t26457 + t26490;
    let t26514 = t5843 * t1058;
    let t26515 = t28 * t26514;
    let t26520 = t6718 * t609;
    (t26493, t26515, t26520)
}
