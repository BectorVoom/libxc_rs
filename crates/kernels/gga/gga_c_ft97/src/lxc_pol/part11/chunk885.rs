//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 885/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk885<F: Float>(t39706: F, t39763: F, t40299: F, t40514: F, t605: F, t2142: F, t9258: F, t604: F, t9394: F, t609: F, t2133: F, t2178: F, t2180: F, t39673: F, t39662: F, t39666: F, t39670: F, t39679: F, t39681: F, t39683: F, t39685: F, t39687: F, t39689: F, t39691: F, t39696: F, t39700: F) -> (F, F, F, F, F) {
    let t40517 = t605 * (t39706 + t39763 + t40299 + t40514);
    let t40519 = t2142 * t9258;
    let t40521 = t9394 * t604;
    let t40522 = t40521 * t609;
    let t40524 = t2133 * t2178;
    let t40525 = t40524 * t2180;
    let t40530 = 140.0 / 243.0 * t39673;
    let t40540 = 4.0 / 9.0 * t39662 - 4.0 / 3.0 * t39666 - 4.0 / 3.0 * t39670 + t40530 - 10.0 / 27.0 * t39679 + 4.0 / 9.0 * t39681 - 2.0 / 9.0 * t39683 - 4.0 / 9.0 * t39685 + 4.0 / 27.0 * t39687 - 4.0 / 27.0 * t39689 + 4.0 / 9.0 * t39691 - 4.0 / 9.0 * t39696 - 8.0 / 9.0 * t39700;
    (t40517, t40519, t40522, t40525, t40540)
}
