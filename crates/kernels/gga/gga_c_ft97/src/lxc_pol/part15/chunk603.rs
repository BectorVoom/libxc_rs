//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 603/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk603<F: Float>(t1546: F, t4660: F, t89: F, t4652: F, t7780: F, t4702: F, t8907: F, t4441: F, t8690: F, t4687: F, t8959: F, t422: F, t1008: F, t132: F, t4698: F, t549: F) -> (F, F, F, F, F, F, F, F) {
    let t16748 = t89 * t1546 * t4660;
    let t16751 = t89 * t7780 * t4652;
    let t16798 = t8907 * t4702;
    let t16832 = t8690 * t4441;
    let t16853 = 0.8854768453090786061e-3 * t8959 * t4687;
    let t16854 = t422 * t4441;
    let t16891 = t1008 * t132;
    let t16907 = t549 * t4698;
    (t16748, t16751, t16798, t16832, t16853, t16854, t16891, t16907)
}
