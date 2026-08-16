//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1069/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1069(t78367: f64, t75907: f64, t75910: f64, t15605: f64, t302: f64, t70063: f64, t70100: f64, t71670: f64, t71671: f64, t71672: f64, t72: f64, t75892: f64, t75895: f64, t78349: f64, t78352: f64, t78355: f64, t78359: f64, t78362: f64, t78364: f64) -> f64 {
    let t78368 = 0.42564599893297839398e-5_f64 * t78367;
    let t78371 = 0.1276937996798935182e-4_f64 * t75907;
    let t78372 = 0.1276937996798935182e-4_f64 * t75910;
    let t78373 = -t78349 - t78352 - t78355 + 0.16566831523319392755e-1_f64 * t75892 - 0.91976356987732177731e-5_f64 * t70063 - 0.20439190441718261719e-5_f64 * t75895 - t71670 - t71671 - t71672 + t78359 - 0.15372131649401827111e-4_f64 * t70100 + t78362 + t78364 - t78368 + t72 * t302 * t15605 + t78371 - t78372;
    t78373
}
