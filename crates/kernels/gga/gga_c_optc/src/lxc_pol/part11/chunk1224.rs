//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1224/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1224<F: Float>(t56164: F, t6: F, t127: F, t22787: F, t6879: F, t2024: F, t1271: F, t13185: F, t161: F, t2021: F, t22786: F, t38486: F, t48260: F, t48262: F, t48272: F, t56124: F, t56128: F, t56132: F, t56136: F, t56140: F, t56145: F, t56149: F, t56154: F, t56159: F, t636: F, t6876: F, t9600: F) -> (F, F, F, F, F) {
    let t56165 = t6 * t56164;
    let t56166 = t56165 * t127;
    let t56170 = t56165 * t22787;
    let t56174 = t56165 * t6879;
    let t56178 = t56165 * t2024;
    let t56187 = -F::cast_from(0.32599355586707841954e0_f64) * t636 * t56124 + F::cast_from(0.65198711173415683908e-1_f64) * t636 * t56128 + F::cast_from(0.26079484469366273564e0_f64) * t6876 * t56132 - F::cast_from(0.97798066760123525865e-1_f64) * t6876 * t56136 + F::cast_from(0.97798066760123525863e-1_f64) * t2021 * t56140 + F::cast_from(0.13039742234683136782e1_f64) * t636 * t56145 - F::cast_from(0.16299677793353920978e-1_f64) * t636 * t56149 - F::cast_from(0.10866451862235947318e-1_f64) * t636 * t56154 + F::cast_from(0.43465807448943789272e-1_f64) * t636 * t56159 + F::cast_from(0.5071010869043442082e-1_f64) * t48260 - F::cast_from(0.30426065214260652492e0_f64) * t48262 - F::cast_from(0.27166129655589868296e-2_f64) * t636 * t161 * t56166 + F::cast_from(0.65198711173415683912e-1_f64) * t22786 * t161 * t56170 - F::cast_from(0.97798066760123525865e-1_f64) * t6876 * t161 * t56174 + F::cast_from(0.38032581517825815613e-1_f64) * t2021 * t161 * t56178 + F::cast_from(0.20284043476173768328e0_f64) * t48272 - F::cast_from(0.26079484469366273564e0_f64) * t9600 * t38486 * t13185 * t1271;
    (t56166, t56170, t56174, t56178, t56187)
}
