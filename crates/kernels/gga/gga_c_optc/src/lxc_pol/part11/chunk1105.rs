//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1105/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1105<F: Float>(t13214: F, t4649: F, t162: F, t127: F, t1271: F, t16370: F, t16287: F, t2034: F, t4623: F, t6: F, t22787: F, t6879: F, t2024: F, t13185: F, t161: F, t2021: F, t22786: F, t38486: F, t48260: F, t48262: F, t48272: F, t56124: F, t56128: F, t56132: F, t56136: F, t56140: F, t56145: F, t636: F, t6876: F, t9600: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t56148 = t13214 * t4649;
    let t56149 = t162 * t56148;
    let t56153 = t16370 * t1271 * t127;
    let t56154 = t162 * t56153;
    let t56158 = t16287 * t1271 * t127;
    let t56159 = t2034 * t56158;
    let t56164 = t4623 * t4623;
    let t56165 = t6 * t56164;
    let t56166 = t56165 * t127;
    let t56170 = t56165 * t22787;
    let t56174 = t56165 * t6879;
    let t56178 = t56165 * t2024;
    let t56187 = -0.32599355586707841954e0 * t636 * t56124 + 0.65198711173415683908e-1 * t636 * t56128 + 0.26079484469366273564e0 * t6876 * t56132 - 0.97798066760123525865e-1 * t6876 * t56136 + 0.97798066760123525863e-1 * t2021 * t56140 + 0.13039742234683136782e1 * t636 * t56145 - 0.16299677793353920978e-1 * t636 * t56149 - 0.10866451862235947318e-1 * t636 * t56154 + 0.43465807448943789272e-1 * t636 * t56159 + 0.5071010869043442082e-1 * t48260 - 0.30426065214260652492e0 * t48262 - 0.27166129655589868296e-2 * t636 * t161 * t56166 + 0.65198711173415683912e-1 * t22786 * t161 * t56170 - 0.97798066760123525865e-1 * t6876 * t161 * t56174 + 0.38032581517825815613e-1 * t2021 * t161 * t56178 + 0.20284043476173768328e0 * t48272 - 0.26079484469366273564e0 * t9600 * t38486 * t13185 * t1271;
    (t56148, t56149, t56153, t56154, t56158, t56159, t56164, t56166, t56170, t56174, t56178, t56187)
}
