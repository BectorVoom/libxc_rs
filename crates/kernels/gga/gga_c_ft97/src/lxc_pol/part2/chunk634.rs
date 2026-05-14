//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 634/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk634<F: Float>(t8051: F, t938: F, t383: F, t7857: F, t1598: F, t66: F, t3021: F, t6: F, t1620: F, t11080: F, t11085: F, t11090: F, t11094: F, t11095: F, t11098: F, t11104: F, t11109: F, t1603: F, t1617: F, t1683: F, t1712: F, t3022: F, t3025: F, t3029: F, t3076: F, t372: F, t374: F, t7838: F, t7839: F, t7845: F, t8009: F, t8015: F, t8018: F, t940: F) -> (F,) {
    let t11115 = t8051 * t938;
    let t11119 = t7857 * t383;
    let t11120 = t1598 * t66;
    let t11121 = t11119 * t11120;
    let t11122 = t3021 * t6;
    let t11123 = t11122 * t1620;
    let t11126 = 0.37540077436335915588e-1 * t940 * t1683 - 0.46509801892875584e-1 * t1603 * t374 * t11080 - 0.279058811357253504e-2 * t372 * t11085 + 0.38731446812548799882e-3 * t372 * t11090 + 0.13784064983740990796e-3 * t11094 * t11095 + 0.91830411319857336051e-5 * t8009 * t11098 - 0.27568129967481981592e-3 * t7838 * t3025 * t7839 + 0.13784064983740990796e-3 * t7845 * t11104 + 0.27568129967481981592e-4 * t8015 * t11098 - 0.40559281352147498558e-4 * t11109 * t3022 + 0.43649539115179804188e-3 * t1617 * t3029 * t8018 - 6.0 * t3076 * t11115 * t1712 + 0.32054706583615839486e-5 * t11121 * t11123;
    (t11126,)
}
