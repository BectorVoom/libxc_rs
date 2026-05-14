//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1057/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1057<F: Float>(t355: F, t7368: F, t525: F, t7760: F, t1554: F, t1984: F, t2133: F, t2178: F, t597: F, t9438: F, t582: F, t605: F, t9132: F, t24: F, t32905: F, t2101: F, t2179: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40424 = t355 * t7368;
    let t40436 = t7760 * t525;
    let t40465 = t1554 * t1984;
    let t40524 = t2133 * t2178;
    let t40591 = t597 * t9438;
    let t40735 = t582 * t2133;
    let t40792 = t9132 * t605;
    let t40830 = t24 * t32905;
    let t40911 = t2101 * t2179;
    (t40424, t40436, t40465, t40524, t40591, t40735, t40792, t40830, t40911)
}
