//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1215/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1215<F: Float>(t26261: F, t26309: F, t26311: F, t26314: F, t26319: F, t26324: F, t26326: F, t26328: F, t26330: F, t26332: F, t26339: F, t26343: F, t26777: F, t415: F, t26252: F, t26258: F, t26278: F, t26280: F, t26284: F, t26289: F, t26293: F, t26296: F, t26300: F, t26304: F, t26306: F) -> (F, F) {
    let t26780 = 0.96141975308641975307e-1 * t26261;
    let t26790 = 0.24722222222222222222e-1 * t26309 - 0.49444444444444444444e-1 * t26311 + t26780 + 0.12361111111111111111e-1 * t26314 + 0.74166666666666666668e-1 * t26319 - 0.24722222222222222222e-1 * t26324 - 0.24722222222222222222e-1 * t26326 - 0.16481481481481481482e-1 * t26328 + 0.49444444444444444445e-1 * t26330 + 0.38456790123456790123e-1 * t26332 - 0.27469135802469135803e-1 * t26339 - 0.92708333333333333333e-2 * t26343;
    let t26792 = (t26777 + t26790) * t415;
    let t26805 = 0.25367901234567901233e-1 * t26252 + 0.2283111111111111111e0 * t26258 - 0.11415555555555555555e0 * t26278 + 0.13698666666666666667e0 * t26280 - 0.41095999999999999999e0 * t26284 + 0.41095999999999999998e0 * t26289 - 0.34246666666666666665e-1 * t26293 + 0.41096e0 * t26296 - 0.61644e0 * t26300 + 0.10274e0 * t26304 - 0.13698666666666666667e0 * t26306;
    (t26792, t26805)
}
