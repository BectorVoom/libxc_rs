//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 972/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk972<F: Float>(t2951: F, t2930: F, t8640: F, t10875: F, t2253: F, t10918: F, t10900: F, t10852: F, t170: F, t328: F, t39600: F, t10850: F, t10904: F, t10915: F, t14514: F, t14519: F, t2265: F, t231: F, t2928: F, t2938: F, t2939: F, t41448: F, t41468: F, t43046: F, t43050: F, t631: F, t898: F) -> (F,) {
    let t43062 = t2951 * t2951;
    let t43072 = t8640 * t2930;
    let t43074 = t2253 * t10875;
    let t43076 = t2253 * t10918;
    let t43078 = t2253 * t10900;
    let t43080 = t2253 * t10852;
    let t43084 = 220.0 / 81.0 * t170 * t39600 * t328;
    let t43088 = 8.0 * t2265 * t14519 * t43046 - 8.0 / 9.0 * t631 * t10915 * t43050 * t41448 - 4.0 * t631 * t231 * t10850 * t41448 - t631 * t231 * t2928 * t41468 - 9.0 / 2.0 * t631 * t898 * t2938 * t43062 + 36.0 * t631 * t898 * t10904 * t2939 * t2951 - 20.0 / 9.0 * t43072 - 8.0 / 3.0 * t43074 - 16.0 / 81.0 * t43076 - 4.0 / 9.0 * t43078 + 8.0 / 9.0 * t43080 - t43084 - 4.0 / 3.0 * t2265 * t14514 * t43046;
    (t43088,)
}
