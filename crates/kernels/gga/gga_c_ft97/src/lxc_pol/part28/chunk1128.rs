//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1128/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1128<F: Float>(t139212: F, t139352: F, t148280: F, t27081: F, t32962: F, t34822: F, t379: F, t23667: F, t5899: F, t32897: F, t32899: F, t3450: F, t36571: F, t637: F) -> (F, F, F, F, F, F) {
    let t148282 = t139212 * t139352 * t148280;
    let t148284 = t32962 * t27081;
    let t148286 = t139212 * t139352 * t148284;
    let t148288 = t34822 * t379;
    let t148290 = t5899 * t23667 * t148288;
    let t148295 = t32897 * t637 * t36571 * t32899 * t3450;
    (t148282, t148284, t148286, t148288, t148290, t148295)
}
