//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1000/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1000(t118: f64, t1307: f64, t794: f64, t3739: f64, t210: f64, t214: f64, t3719: f64, t116: f64, t534: f64, t212: f64, t2586: f64, t1315: f64, t3725: f64, t3727: f64, t3731: f64, t3733: f64, t3736: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3741 = t118 * t794 * t1307;
    let t3742 = t3739 * t3741;
    let t3745 = t210 * t214 * t3719;
    let t3748 = t534 * t116;
    let t3749 = t3748 * t212;
    let t3751 = 0.83333333333333333332e-3_f64 * t2586 * t3749;
    let t3752 = t3725 + 0.77777777777777777775e-2_f64 * t3727 + t3731 + 0.49999999999999999998e-2_f64 * t3733 * t3736 + 0.16666666666666666666e-2_f64 * t3742 - 0.16666666666666666666e-2_f64 * t1315 * t3745 - t3751;
    (t3741, t3742, t3745, t3749, t3751, t3752)
}
