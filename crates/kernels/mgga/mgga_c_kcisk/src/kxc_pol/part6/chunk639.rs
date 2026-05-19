//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 639/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk639<F: Float>(t1791: F, t8845: F, t2469: F, t4826: F, t8831: F, t719: F, t717: F, t415: F, t2509: F, t2533: F, t1693: F, t2470: F, t4809: F, t4823: F, t6949: F, t6951: F, t6959: F, t7278: F, t8482: F, t8487: F, t8668: F, t8675: F, t8679: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8846 = t8845 * t1791;
    let t8851 = t2469 * t2469;
    let t8852 = t8851 * t4826;
    let t8857 = sigma2 * t8831;
    let t8858 = t8857 * t719;
    let t8859 = t717 * t8858;
    let t8860 = t415 * t8859;
    let t8862 = t2509 * t2533;
    let t8863 = t415 * t8862;
    let t8865 = -t4809 - F::cast_from(0.33163888888888888888e-2_f64) * t8482 + F::cast_from(0.22109259259259259258e-2_f64) * t8487 + F::cast_from(0.24872916666666666666e-2_f64) * t8668 + F::cast_from(0.22109259259259259258e-2_f64) * t6949 - F::cast_from(0.33163888888888888888e-2_f64) * t6951 + F::cast_from(0.49745833333333333332e-2_f64) * t8675 + F::cast_from(0.13265555555555555555e-1_f64) * t8679 + F::cast_from(0.22109259259259259258e-2_f64) * t6959 - F::new(0.193e0) * t1693 * t8846 - F::new(0.386e0) * t7278 * t2470 + F::new(0.193e0) * t1693 * t8852 + F::new(0.74498e-1) * t4823 * t8852 + F::cast_from(0.24320185185185185185e-1_f64) * t8860 - F::cast_from(0.13265555555555555555e-1_f64) * t8863;
    (t8846, t8851, t8852, t8857, t8858, t8859, t8860, t8862, t8863, t8865)
}
