//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 639/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk639(t1791: f64, t8845: f64, t2469: f64, t4826: f64, t8831: f64, t719: f64, t717: f64, t415: f64, t2509: f64, t2533: f64, t1693: f64, t2470: f64, t4809: f64, t4823: f64, t6949: f64, t6951: f64, t6959: f64, t7278: f64, t8482: f64, t8487: f64, t8668: f64, t8675: f64, t8679: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8846 = t8845 * t1791;
    let t8851 = t2469 * t2469;
    let t8852 = t8851 * t4826;
    let t8857 = sigma2 * t8831;
    let t8858 = t8857 * t719;
    let t8859 = t717 * t8858;
    let t8860 = t415 * t8859;
    let t8862 = t2509 * t2533;
    let t8863 = t415 * t8862;
    let t8865 = -t4809 - 0.33163888888888888888e-2_f64 * t8482 + 0.22109259259259259258e-2_f64 * t8487 + 0.24872916666666666666e-2_f64 * t8668 + 0.22109259259259259258e-2_f64 * t6949 - 0.33163888888888888888e-2_f64 * t6951 + 0.49745833333333333332e-2_f64 * t8675 + 0.13265555555555555555e-1_f64 * t8679 + 0.22109259259259259258e-2_f64 * t6959 - 0.193e0_f64 * t1693 * t8846 - 0.386e0_f64 * t7278 * t2470 + 0.193e0_f64 * t1693 * t8852 + 0.74498e-1_f64 * t4823 * t8852 + 0.24320185185185185185e-1_f64 * t8860 - 0.13265555555555555555e-1_f64 * t8863;
    (t8846, t8851, t8852, t8857, t8858, t8859, t8860, t8862, t8863, t8865)
}
