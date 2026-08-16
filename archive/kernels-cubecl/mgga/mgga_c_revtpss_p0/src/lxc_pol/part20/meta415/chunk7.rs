//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1542/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1542<F: Float>(t378: F, t42385: F, t16565: F, t994: F, t11247: F, t999: F, t42859: F, t42862: F, t342: F, t42869: F, t3145: F, t368: F) -> (F, F, F, F, F, F) {
    let t43334 = t378 * t42385;
    let t43341 = t994 * t16565;
    let t43342 = t999 * t11247;
    let t43346 = t42859 * t42862;
    let t43347 = t342 * t43346;
    let t43348 = t378 * t42869;
    let t43350 = F::cast_from(1.0_f64) / t3145 / t368;
    (t43334, t43341, t43342, t43347, t43348, t43350)
}
