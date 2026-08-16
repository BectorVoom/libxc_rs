//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 904/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk904<F: Float>(t480: F, t8216: F, t159: F, t9437: F, t157: F, t1642: F, t1984: F, t378: F, t7368: F, t137: F, t8906: F, t542: F, t7334: F) -> (F, F, F, F, F, F, F) {
    let t39150 = t8216 * t480;
    let t39652 = F::cast_from(1.0_f64) / t9437 / t159;
    let t39653 = t157 * t39652;
    let t39693 = t1642 * t1984;
    let t39749 = t378 * t7368;
    let t39801 = F::cast_from(1.0_f64) / t8906 / t137;
    let t39852 = t542 * t7334;
    (t39150, t39652, t39653, t39693, t39749, t39801, t39852)
}
