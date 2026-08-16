//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1224/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1224(t56164: f64, t6: f64, t127: f64, t22787: f64, t6879: f64, t2024: f64, t1271: f64, t13185: f64, t161: f64, t2021: f64, t22786: f64, t38486: f64, t48260: f64, t48262: f64, t48272: f64, t56124: f64, t56128: f64, t56132: f64, t56136: f64, t56140: f64, t56145: f64, t56149: f64, t56154: f64, t56159: f64, t636: f64, t6876: f64, t9600: f64) -> (f64, f64, f64, f64, f64) {
    let t56165 = t6 * t56164;
    let t56166 = t56165 * t127;
    let t56170 = t56165 * t22787;
    let t56174 = t56165 * t6879;
    let t56178 = t56165 * t2024;
    let t56187 = -0.32599355586707841954e0_f64 * t636 * t56124 + 0.65198711173415683908e-1_f64 * t636 * t56128 + 0.26079484469366273564e0_f64 * t6876 * t56132 - 0.97798066760123525865e-1_f64 * t6876 * t56136 + 0.97798066760123525863e-1_f64 * t2021 * t56140 + 0.13039742234683136782e1_f64 * t636 * t56145 - 0.16299677793353920978e-1_f64 * t636 * t56149 - 0.10866451862235947318e-1_f64 * t636 * t56154 + 0.43465807448943789272e-1_f64 * t636 * t56159 + 0.5071010869043442082e-1_f64 * t48260 - 0.30426065214260652492e0_f64 * t48262 - 0.27166129655589868296e-2_f64 * t636 * t161 * t56166 + 0.65198711173415683912e-1_f64 * t22786 * t161 * t56170 - 0.97798066760123525865e-1_f64 * t6876 * t161 * t56174 + 0.38032581517825815613e-1_f64 * t2021 * t161 * t56178 + 0.20284043476173768328e0_f64 * t48272 - 0.26079484469366273564e0_f64 * t9600 * t38486 * t13185 * t1271;
    (t56166, t56170, t56174, t56178, t56187)
}
