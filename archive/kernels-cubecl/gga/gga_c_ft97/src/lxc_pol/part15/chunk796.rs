//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 796/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk796<F: Float>(t21639: F, t762: F, t242: F, t1131: F, t4635: F, t2600: F, t2599: F, t1168: F, t2607: F, t2606: F, t992: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21640 = t762 * t21639;
    let t21641 = t242 * t21640;
    let t21645 = t4635 * t1131;
    let t21646 = t2600 * t21645;
    let t21647 = t2599 * t21646;
    let t21650 = t4635 * t1168;
    let t21651 = t2607 * t21650;
    let t21652 = t2606 * t21651;
    let t21655 = t4635 * t992;
    (t21640, t21641, t21645, t21646, t21647, t21650, t21651, t21652, t21655)
}
