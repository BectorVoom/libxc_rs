//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 750/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk750(t6810: f64, t6842: f64, t582: f64, t186: f64, t211: f64, t1960: f64, t835: f64, t549: f64, t820: f64, t184: f64, t813: f64, t4041: f64, t5057: f64, t5172: f64, t5179: f64, t5186: f64, t5190: f64, t5192: f64, t5194: f64, t5198: f64, t5200: f64, t6785: f64, t6786: f64, t6790: f64, t6792: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6843 = t6810 + t6842;
    let t6844 = t582 * t6843;
    let t6845 = t186 * t6844;
    let t6847 = 2.0_f64 / 15.0_f64 * t211 * t6845;
    let t6849 = 4.0_f64 / 15.0_f64 * t1960 * t835;
    let t6850 = t549 * t820;
    let t6851 = t6850 * t184;
    let t6853 = 8.0_f64 / 15.0_f64 * t6851 * t813;
    let t6854 = -t5057 - t5172 - t6785 + t5179 - t6786 + t6790 + t4041 + t6792 - t5186 + t5190 + t5192 + t5194 - t5198 + t5200 - t6847 - t6849 + t6853;
    (t6843, t6844, t6845, t6847, t6849, t6850, t6851, t6853, t6854)
}
