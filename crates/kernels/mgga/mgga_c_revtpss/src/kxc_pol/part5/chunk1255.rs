//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1255/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1255<F: Float>(t1045: F, t4579: F, t15691: F, t1043: F, t1592: F, t3155: F, t4817: F, t4834: F, t11933: F, t11956: F, t11967: F, t11972: F, t11989: F, t15700: F, t15830: F, t16121: F, t16226: F, t1675: F, t3211: F, t6273: F, t6278: F) -> F {
    let t19992 = t1045 * t4579;
    let t19993 = t15691 * t19992;
    let t19996 = t1592 * t1043;
    let t19997 = t3155 * t19996;
    let t19998 = t15691 * t19997;
    let t20005 = t4834 * t4817;
    let t20012 = -F::new(0.57165357490759649296e-3) * t15700 * t19993 + F::new(0.57165357490759649296e-3) * t16226 * t19998 - F::new(0.47637797908966374413e-4) * t11956 + F::new(0.2540682555144873302e-3) * t11967 + t11972 - F::new(0.15244095330869239812e-2) * t15830 * t1675 + F::new(0.19055119163586549765e-3) * t20005 - F::new(0.31758531939310916275e-4) * t11989 - t16121 + F::new(0.22866142996303859718e-2) * t11933 * t6273 + F::new(0.11433071498151929859e-2) * t3211 * t6278;
    t20012
}
