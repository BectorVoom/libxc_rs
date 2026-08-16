//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 848/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk848(t8705: f64, t8718: f64, t974: f64, t89: f64, t2152: f64, t623: f64, t844: f64, t164: f64, t1011: f64, t2426: f64, t4623: f64, t4625: f64, t4627: f64, t709: f64, t7792: f64, t8679: f64, t8682: f64, t8684: f64, t8686: f64, t8689: f64, t8691: f64, t98: f64) -> (f64, f64, f64) {
    let t8719 = t8705 + t8718;
    let t8720 = t8719 * t974;
    let t8721 = t8720 * t89;
    let t8724 = t2152 * t623;
    let t8725 = t844 * t8724;
    let t8726 = t164 * t8725;
    let t8730 = 12.992782516386768_f64 * t8679 - t4623 + t4625 + t4627 + 3.159189221415045_f64 * t8682 - 1.6183441301295518_f64 * t8684 - 1.6183441301295518_f64 * t8686 + 0.7897973053537612_f64 * t8689 + 1.6183441301295518_f64 * t8691 + 19.489173774580152_f64 * t2426 * t1011 + 19.489173774580152_f64 * t8721 * t98 + 22.07984838129906_f64 * t8726 + 2.427516195194328_f64 * t7792 * t709;
    (t8720, t8726, t8730)
}
