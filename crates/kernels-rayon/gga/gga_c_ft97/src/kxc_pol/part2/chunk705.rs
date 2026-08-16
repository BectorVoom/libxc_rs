//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 705/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk705(t383: f64, t7857: f64, t1598: f64, t66: f64, t3021: f64, t6: f64, t1620: f64, t11080: f64, t11085: f64, t11090: f64, t11094: f64, t11095: f64, t11098: f64, t11104: f64, t11109: f64, t11115: f64, t1603: f64, t1617: f64, t1683: f64, t1712: f64, t3022: f64, t3025: f64, t3029: f64, t3076: f64, t372: f64, t374: f64, t7838: f64, t7839: f64, t7845: f64, t8009: f64, t8015: f64, t8018: f64, t940: f64) -> f64 {
    let t11119 = t7857 * t383;
    let t11120 = t1598 * t66;
    let t11121 = t11119 * t11120;
    let t11122 = t3021 * t6;
    let t11123 = t11122 * t1620;
    let t11126 = 0.37540077436335915588e-1_f64 * t940 * t1683 - 0.46509801892875584e-1_f64 * t1603 * t374 * t11080 - 0.279058811357253504e-2_f64 * t372 * t11085 + 0.38731446812548799882e-3_f64 * t372 * t11090 + 0.13784064983740990796e-3_f64 * t11094 * t11095 + 0.91830411319857336051e-5_f64 * t8009 * t11098 - 0.27568129967481981592e-3_f64 * t7838 * t3025 * t7839 + 0.13784064983740990796e-3_f64 * t7845 * t11104 + 0.27568129967481981592e-4_f64 * t8015 * t11098 - 0.40559281352147498558e-4_f64 * t11109 * t3022 + 0.43649539115179804188e-3_f64 * t1617 * t3029 * t8018 - 6.0_f64 * t3076 * t11115 * t1712 + 0.32054706583615839486e-5_f64 * t11121 * t11123;
    t11126
}
