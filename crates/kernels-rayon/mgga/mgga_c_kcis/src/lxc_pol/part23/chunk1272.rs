//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1272/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1272(t16795: f64, t27387: f64, t4153: f64, t1394: f64, t15828: f64, t7923: f64, t15834: f64, t1386: f64, t1598: f64, t2237: f64, t2239: f64, t27403: f64, t27416: f64, t498: f64, t51602: f64, t531: f64, t6163: f64, t8144: f64, t8151: f64, t94638: f64, t98767: f64, t98777: f64, t98781: f64) -> (f64, f64, f64, f64) {
    let t98784 = t4153 * t27387 * t16795;
    let t98787 = t1394 * t7923 * t15828;
    let t98790 = t4153 * t7923 * t15834;
    let t98792 = -0.69505208333333333333e-3_f64 * t51602 * t1598 * t2239 + 0.69505208333333333333e-3_f64 * t8144 * t27403 + 0.24872916666666666666e-2_f64 * t98767 - 0.18534722222222222222e-2_f64 * t8151 * t27416 + 0.46336805555555555556e-3_f64 * t2237 * t6163 * t498 * t1386 * t531 - 0.15445601851851851852e-3_f64 * t98777 + 0.15445601851851851852e-3_f64 * t94638 - 0.16581944444444444444e-2_f64 * t98781 - 0.27636574074074074073e-2_f64 * t98784 + 0.11054629629629629629e-2_f64 * t98787 + 0.18424382716049382715e-2_f64 * t98790;
    (t98784, t98787, t98790, t98792)
}
