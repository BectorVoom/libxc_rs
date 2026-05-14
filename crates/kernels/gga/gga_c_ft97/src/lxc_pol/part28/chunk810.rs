//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 810/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk810<F: Float>(t100: F, t1586: F, t10: F, t16: F, t1642: F, t369: F, t2035: F, t39: F, t538: F, t355: F, t929: F, t526: F, t597: F, t2178: F, t3539: F, t1045: F, t9132: F) -> (F, F, F, F, F, F, F, F) {
    let t47660 = t1586 * t100;
    let t47666 = t10 * t16 * t1642;
    let t47667 = t369 * t100;
    let t48841 = t538 * t39 * t2035;
    let t48917 = t355 * t929;
    let t49414 = t526 * t597;
    let t49562 = t3539 * t2178;
    let t49622 = t9132 * t1045;
    (t47660, t47666, t47667, t48841, t48917, t49414, t49562, t49622)
}
