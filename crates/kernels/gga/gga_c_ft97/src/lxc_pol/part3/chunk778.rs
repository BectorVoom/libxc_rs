//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 778/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk778<F: Float>(t17766: F, t2594: F, t446: F, t5053: F, t668: F, t505: F, t2354: F, t4934: F, t9770: F, t18: F, t3699: F) -> (F, F, F, F, F, F) {
    let t17767 = t2594 * t17766;
    let t17768 = t446 * t17767;
    let t17770 = t5053 * t668;
    let t17771 = t17770 * t505;
    let t17772 = t2354 * t17771;
    let t17773 = t446 * t17772;
    let t17775 = t4934 * t668;
    let t17776 = t17775 * t505;
    let t17777 = t9770 * t17776;
    let t17778 = t446 * t17777;
    let t17780 = t3699 * t18;
    (t17768, t17771, t17773, t17776, t17778, t17780)
}
