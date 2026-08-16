//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3490/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3490(t1065: f64, t19380: f64, t1062: f64, t19463: f64, t1042: f64, t1063: f64, t11994: f64, t15791: f64, t15938: f64, t16196: f64, t16201: f64, t19668: f64, t19677: f64, t19930: f64, t19968: f64, t3101: f64, t3106: f64, t3127: f64, t3130: f64, t4806: f64, t4834: f64, t53393: f64, t60834: f64, t60838: f64, t906: f64) -> f64 {
    let t65712 = t1065 * t19380;
    let t65717 = t19463 * t1062;
    let t65727 = -0.11433071498151929859e-2_f64 * t4834 * t15791 - 0.57165357490759649296e-3_f64 * t4834 * t16196 + 0.17149607247227894789e-2_f64 * t4834 * t15938 - 0.28582678745379824648e-2_f64 * t4834 * t16201 + 0.47637797908966374414e-3_f64 * t1063 * t1042 * t4806 * t60838 + 0.23818898954483187207e-3_f64 * t1063 * t1042 * t4806 * t60834 - 0.28582678745379824648e-3_f64 * t11994 * t19677 - 0.28582678745379824648e-3_f64 * t3127 * t1042 * t65712 * t906 - 0.28582678745379824648e-3_f64 * t65717 * t3130 - 0.91464571985215438873e-2_f64 * t3106 * t19930 - 0.50813651102897466041e-2_f64 * t3106 * t19668 - 0.28582678745379824648e-3_f64 * t19968 * t3101 - 0.3811023832717309953e-3_f64 * t53393;
    t65727
}
