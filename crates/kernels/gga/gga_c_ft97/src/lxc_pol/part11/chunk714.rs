//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 714/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk714<F: Float>(t89: F, t9716: F, t9718: F, t2336: F, t2366: F, t2344: F, t375: F, t2350: F, t2374: F, t9520: F, t9695: F, t9699: F, t9701: F, t9705: F, t9711: F, t9715: F) -> (F, F, F, F, F, F) {
    let t9720 = t89 * t9716 * t9718;
    let t9723 = t89 * t2336 * t2366;
    let t9725 = t375 * t2344;
    let t9727 = t89 * t9725 * t2350;
    let t9730 = t89 * t375 * t2374;
    let t9732 = t9520 / F::new(6.0) - t9695 / F::new(6.0) - t9699 - F::new(2.0) / F::new(9.0) * t9701 - t9705 / F::new(18.0) - t9711 + t9715 - F::new(5.0) / F::new(81.0) * t9720 + t9723 / F::new(18.0) + t9727 / F::new(27.0) - t9730 / F::new(3.0);
    (t9720, t9723, t9725, t9727, t9730, t9732)
}
