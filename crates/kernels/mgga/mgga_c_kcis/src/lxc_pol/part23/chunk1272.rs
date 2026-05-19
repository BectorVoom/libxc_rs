//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1272/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1272<F: Float>(t16795: F, t27387: F, t4153: F, t1394: F, t15828: F, t7923: F, t15834: F, t1386: F, t1598: F, t2237: F, t2239: F, t27403: F, t27416: F, t498: F, t51602: F, t531: F, t6163: F, t8144: F, t8151: F, t94638: F, t98767: F, t98777: F, t98781: F) -> (F, F, F, F) {
    let t98784 = t4153 * t27387 * t16795;
    let t98787 = t1394 * t7923 * t15828;
    let t98790 = t4153 * t7923 * t15834;
    let t98792 = -F::cast_from(0.69505208333333333333e-3_f64) * t51602 * t1598 * t2239 + F::cast_from(0.69505208333333333333e-3_f64) * t8144 * t27403 + F::cast_from(0.24872916666666666666e-2_f64) * t98767 - F::cast_from(0.18534722222222222222e-2_f64) * t8151 * t27416 + F::cast_from(0.46336805555555555556e-3_f64) * t2237 * t6163 * t498 * t1386 * t531 - F::cast_from(0.15445601851851851852e-3_f64) * t98777 + F::cast_from(0.15445601851851851852e-3_f64) * t94638 - F::cast_from(0.16581944444444444444e-2_f64) * t98781 - F::cast_from(0.27636574074074074073e-2_f64) * t98784 + F::cast_from(0.11054629629629629629e-2_f64) * t98787 + F::cast_from(0.18424382716049382715e-2_f64) * t98790;
    (t98784, t98787, t98790, t98792)
}
