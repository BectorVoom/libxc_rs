//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 241/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk241<F: Float>(t206: F, t220: F, t62: F, t691: F, t20: F, t212: F, t870: F, t209: F, t208: F, t214: F, t217: F, t221: F, t712: F, t750: F, t777: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t210 = F::new(0.0) < t206;
    let t872 = t220 * t220;
    let t873 = F::new(1.0) / t872;
    let t874 = t206 * t873;
    let t875 = t62 * t691;
    let t876 = t875 * t20;
    let t879 = t212 * t212;
    let t880 = F::new(1.0) / t879;
    let t882 = piecewise3::<F>(t210, t870, -t870);
    let t884 = t209 * t880 * t882;
    let t887 = -F::new(7.0) / F::new(288.0) * t876 * t214 - t208 * t884 / F::new(96.0);
    let t888 = F::new(1.0) / t217;
    let t889 = t887 * t888;
    let t895 = t870 * t221 - F::new(0.66725e-1) * t874 * t889 - F::cast_from(0.92858888888888888886e-2_f64) * t712 + F::cast_from(0.69644166666666666665e-2_f64) * t750 - F::cast_from(0.69644166666666666665e-2_f64) * t777;
    (t872, t873, t874, t876, t879, t880, t882, t884, t887, t888, t889, t895)
}
