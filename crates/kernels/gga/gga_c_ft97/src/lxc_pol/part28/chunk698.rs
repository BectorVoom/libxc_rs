//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 698/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk698<F: Float>(t1882: F, t7276: F, t1851: F, t7274: F, t379: F, t1909: F, t1901: F, t32429: F, t32433: F, t32459: F, t32463: F, t32465: F, t32469: F, t32471: F, t32475: F, t32479: F, t32483: F, t446: F) -> (F, F, F, F, F) {
    let t32487 = 2.0 / 9.0 * t1882 * t7276;
    let t32488 = t1851 * t7274;
    let t32489 = t32488 * t379;
    let t32490 = t1909 * t32489;
    let t32493 = -2.0 / 3.0 * t446 * t32429 + t32433 - t446 * t32459 / 3.0 + t32463 - t446 * t32465 / 3.0 + t32469 - t446 * t32471 / 3.0 + 2.0 / 3.0 * t446 * t32475 - t446 * t32479 / 3.0 - t446 * t32483 / 3.0 - t32487 - 2.0 / 9.0 * t1901 * t32490;
    (t32487, t32488, t32489, t32490, t32493)
}
