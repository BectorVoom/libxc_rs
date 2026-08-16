//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 913/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk913(t1334: f64, t6986: f64, t3861: f64, t3901: f64, t6985: f64, t3899: f64, t5573: f64, t5577: f64, t11516: f64, t6953: f64, t11513: f64, t11409: f64, t11557: f64, t16046: f64, t16048: f64, t16051: f64, t16052: f64, t21186: f64, t21188: f64, t21193: f64, t21196: f64, t21206: f64, t21209: f64, t21212: f64, t21234: f64, t21237: f64, t21240: f64, t21243: f64) -> (f64, f64, f64, f64, f64) {
    let t21363 = t6986 * t1334;
    let t21365 = 2.0_f64 * t3861 * t21363;
    let t21366 = t6985 * t3901;
    let t21367 = t21366 * t1334;
    let t21369 = 0.16081824322151104822e2_f64 * t3899 * t21367;
    let t21370 = t5577 * t5573;
    let t21372 = 0.32163648644302209644e2_f64 * t3899 * t21370;
    let t21373 = t6953 * t11516;
    let t21374 = t21373 * t1334;
    let t21376 = 0.51725014705706168417e3_f64 * t11513 * t21374;
    let t21400 = -t11557 - 0.79148148148148148147e-2_f64 * t11409 - 0.15829629629629629629e-1_f64 * t16046 + 0.79148148148148148147e-2_f64 * t16048 - t16051 - 0.23744444444444444444e-1_f64 * t16052 + 0.39574074074074074073e-2_f64 * t21186 - 0.19787037037037037037e-1_f64 * t21237 + 0.71233333333333333332e-1_f64 * t21234 + 0.47488888888888888888e-1_f64 * t21240 - 0.11872222222222222222e-1_f64 * t21188 - 0.10685e0_f64 * t21243 - 0.14246666666666666666e0_f64 * t21206 + 0.5936111111111111111e-2_f64 * t21196 - 0.11872222222222222222e-1_f64 * t21209 + 0.35616666666666666666e-1_f64 * t21212 - 0.17808333333333333333e-1_f64 * t21193;
    (t21365, t21369, t21372, t21376, t21400)
}
