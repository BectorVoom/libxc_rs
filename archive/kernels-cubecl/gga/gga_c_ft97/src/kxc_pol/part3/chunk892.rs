//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 892/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk892<F: Float>(t17780: F, t724: F, t3281: F, t1091: F, t3837: F, t9770: F, t446: F, t1131: F, t505: F, t3699: F, t2354: F, t3690: F) -> (F, F, F, F, F, F) {
    let t17781 = t724 * t17780;
    let t17782 = t3281 * t17781;
    let t17785 = t1091 * t3837;
    let t17786 = t9770 * t17785;
    let t17787 = t446 * t17786;
    let t17789 = t1131 * t505;
    let t17790 = t3699 * t17789;
    let t17791 = t2354 * t17790;
    let t17792 = t446 * t17791;
    let t17794 = t3690 * t17789;
    (t17782, t17785, t17787, t17790, t17792, t17794)
}
