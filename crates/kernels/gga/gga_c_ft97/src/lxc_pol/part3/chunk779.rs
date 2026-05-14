//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 779/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk779<F: Float>(t17780: F, t724: F, t3281: F, t1091: F, t3837: F, t9770: F, t446: F, t1131: F, t505: F, t3699: F, t2354: F, t3690: F, t9744: F, t17720: F, t17724: F, t17729: F, t17734: F, t17738: F, t17742: F, t17746: F, t17751: F, t17755: F, t17759: F, t17763: F, t17768: F, t17773: F, t17778: F, t9701: F, t9735: F) -> (F, F, F, F, F, F, F, F) {
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
    let t17795 = t9744 * t17794;
    let t17796 = t446 * t17795;
    let t17799 = -t17720 / 27.0 + t17724 / 18.0 + t17729 / 9.0 - t17734 / 27.0 - 2.0 / 9.0 * t17738 - t17742 / 9.0 - t17746 / 3.0 - 5.0 / 81.0 * t17751 + 4.0 / 27.0 * t17755 + t17759 / 9.0 + t17763 / 27.0 + 2.0 / 9.0 * t17768 + t17773 / 18.0 - t17778 / 9.0 - 4.0 / 9.0 * t17782 - 2.0 / 81.0 * t9735 - 2.0 / 9.0 * t17787 - 2.0 / 9.0 * t17792 + 2.0 / 27.0 * t17796 - 2.0 / 27.0 * t9701;
    (t17782, t17785, t17787, t17790, t17792, t17794, t17796, t17799)
}
