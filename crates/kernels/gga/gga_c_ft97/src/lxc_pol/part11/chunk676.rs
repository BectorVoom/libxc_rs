//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 676/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk676<F: Float>(t2469: F, t2579: F, t729: F, t1934: F, t766: F, t2607: F, t2606: F, t505: F) -> (F, F, F, F) {
    let t9845 = t729 * t2469 * t2579;
    let t9848 = t1934 * t766;
    let t9849 = t2607 * t9848;
    let t9850 = t2606 * t9849;
    let t9853 = t1934 * t505;
    (t9845, t9849, t9850, t9853)
}
