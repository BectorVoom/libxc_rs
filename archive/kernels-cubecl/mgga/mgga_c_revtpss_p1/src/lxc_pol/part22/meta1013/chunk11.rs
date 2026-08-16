//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3490/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3490<F: Float>(t1065: F, t19380: F, t1062: F, t19463: F, t1042: F, t1063: F, t11994: F, t15791: F, t15938: F, t16196: F, t16201: F, t19668: F, t19677: F, t19930: F, t19968: F, t3101: F, t3106: F, t3127: F, t3130: F, t4806: F, t4834: F, t53393: F, t60834: F, t60838: F, t906: F) -> F {
    let t65712 = t1065 * t19380;
    let t65717 = t19463 * t1062;
    let t65727 = -F::cast_from(0.11433071498151929859e-2_f64) * t4834 * t15791 - F::cast_from(0.57165357490759649296e-3_f64) * t4834 * t16196 + F::cast_from(0.17149607247227894789e-2_f64) * t4834 * t15938 - F::cast_from(0.28582678745379824648e-2_f64) * t4834 * t16201 + F::cast_from(0.47637797908966374414e-3_f64) * t1063 * t1042 * t4806 * t60838 + F::cast_from(0.23818898954483187207e-3_f64) * t1063 * t1042 * t4806 * t60834 - F::cast_from(0.28582678745379824648e-3_f64) * t11994 * t19677 - F::cast_from(0.28582678745379824648e-3_f64) * t3127 * t1042 * t65712 * t906 - F::cast_from(0.28582678745379824648e-3_f64) * t65717 * t3130 - F::cast_from(0.91464571985215438873e-2_f64) * t3106 * t19930 - F::cast_from(0.50813651102897466041e-2_f64) * t3106 * t19668 - F::cast_from(0.28582678745379824648e-3_f64) * t19968 * t3101 - F::cast_from(0.3811023832717309953e-3_f64) * t53393;
    t65727
}
