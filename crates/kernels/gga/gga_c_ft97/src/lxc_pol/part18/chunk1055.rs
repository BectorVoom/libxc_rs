//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1055/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1055<F: Float>(t1786: F, t1841: F, t1852: F, t488: F, t8216: F, t8326: F, t463: F, t70: F, t8119: F, t159: F, t9437: F, t157: F, t1642: F, t1984: F, t525: F, t7954: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t38956 = t1786 * t1841;
    let t39107 = t1786 * t1852;
    let t39120 = t8216 * t488;
    let t39167 = t8326 * t488;
    let t39285 = t463 * t1841;
    let t39430 = t70 * t8119;
    let t39652 = 1.0 / t9437 / t159;
    let t39653 = t157 * t39652;
    let t39693 = t1642 * t1984;
    let t39725 = t7954 * t525;
    (t38956, t39107, t39120, t39167, t39285, t39430, t39652, t39653, t39693, t39725)
}
