//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 239/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk239<F: Float>(t206: F, t220: F, t62: F, t691: F, t20: F, t212: F, t870: F, t209: F, t208: F, t214: F, t217: F, t221: F, t712: F, t750: F, t777: F, t227: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t210 = 0.0 < t206;
    let t872 = t220 * t220;
    let t873 = 1.0 / t872;
    let t874 = t206 * t873;
    let t875 = t62 * t691;
    let t876 = t875 * t20;
    let t879 = t212 * t212;
    let t880 = 1.0 / t879;
    let t882 = piecewise3(t210, t870, -t870);
    let t884 = t209 * t880 * t882;
    let t887 = -7.0 / 288.0 * t876 * t214 - t208 * t884 / 96.0;
    let t888 = 1.0 / t217;
    let t889 = t887 * t888;
    let t895 = t870 * t221 - 0.66725e-1 * t874 * t889 - 0.92858888888888888886e-2 * t712 + 0.69644166666666666665e-2 * t750 - 0.69644166666666666665e-2 * t777;
    let t897 = t227 * t227;
    (t872, t873, t874, t876, t879, t880, t882, t884, t887, t888, t889, t895, t897)
}
