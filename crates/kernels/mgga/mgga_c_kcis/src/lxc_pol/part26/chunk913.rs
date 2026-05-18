//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 913/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk913<F: Float>(t1334: F, t6986: F, t3861: F, t3901: F, t6985: F, t3899: F, t5573: F, t5577: F, t11516: F, t6953: F, t11513: F, t11409: F, t11557: F, t16046: F, t16048: F, t16051: F, t16052: F, t21186: F, t21188: F, t21193: F, t21196: F, t21206: F, t21209: F, t21212: F, t21234: F, t21237: F, t21240: F, t21243: F) -> (F, F, F, F, F) {
    let t21363 = t6986 * t1334;
    let t21365 = F::new(2.0) * t3861 * t21363;
    let t21366 = t6985 * t3901;
    let t21367 = t21366 * t1334;
    let t21369 = F::new(0.16081824322151104822e2) * t3899 * t21367;
    let t21370 = t5577 * t5573;
    let t21372 = F::new(0.32163648644302209644e2) * t3899 * t21370;
    let t21373 = t6953 * t11516;
    let t21374 = t21373 * t1334;
    let t21376 = F::new(0.51725014705706168417e3) * t11513 * t21374;
    let t21400 = -t11557 - F::new(0.79148148148148148147e-2) * t11409 - F::new(0.15829629629629629629e-1) * t16046 + F::new(0.79148148148148148147e-2) * t16048 - t16051 - F::new(0.23744444444444444444e-1) * t16052 + F::new(0.39574074074074074073e-2) * t21186 - F::new(0.19787037037037037037e-1) * t21237 + F::new(0.71233333333333333332e-1) * t21234 + F::new(0.47488888888888888888e-1) * t21240 - F::new(0.11872222222222222222e-1) * t21188 - F::new(0.10685e0) * t21243 - F::new(0.14246666666666666666e0) * t21206 + F::new(0.5936111111111111111e-2) * t21196 - F::new(0.11872222222222222222e-1) * t21209 + F::new(0.35616666666666666666e-1) * t21212 - F::new(0.17808333333333333333e-1) * t21193;
    (t21365, t21369, t21372, t21376, t21400)
}
