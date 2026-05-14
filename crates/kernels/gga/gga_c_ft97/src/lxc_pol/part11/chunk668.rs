//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 668/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk668<F: Float>(t2350: F, t89: F, t9725: F, t2374: F, t375: F, t9520: F, t9695: F, t9699: F, t9701: F, t9705: F, t9711: F, t9715: F, t9720: F, t9723: F, t1636: F, t665: F) -> (F, F, F, F) {
    let t9727 = t89 * t9725 * t2350;
    let t9730 = t89 * t375 * t2374;
    let t9732 = t9520 / 6.0 - t9695 / 6.0 - t9699 - 2.0 / 9.0 * t9701 - t9705 / 18.0 - t9711 + t9715 - 5.0 / 81.0 * t9720 + t9723 / 18.0 + t9727 / 27.0 - t9730 / 3.0;
    let t9733 = t1636 * t665;
    (t9727, t9730, t9732, t9733)
}
