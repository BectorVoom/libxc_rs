//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1000/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1000<F: Float>(t118: F, t1307: F, t794: F, t3739: F, t210: F, t214: F, t3719: F, t116: F, t534: F, t212: F, t2586: F, t1315: F, t3725: F, t3727: F, t3731: F, t3733: F, t3736: F) -> (F, F, F, F, F, F) {
    let t3741 = t118 * t794 * t1307;
    let t3742 = t3739 * t3741;
    let t3745 = t210 * t214 * t3719;
    let t3748 = t534 * t116;
    let t3749 = t3748 * t212;
    let t3751 = F::cast_from(0.83333333333333333332e-3_f64) * t2586 * t3749;
    let t3752 = t3725 + F::cast_from(0.77777777777777777775e-2_f64) * t3727 + t3731 + F::cast_from(0.49999999999999999998e-2_f64) * t3733 * t3736 + F::cast_from(0.16666666666666666666e-2_f64) * t3742 - F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t3745 - t3751;
    (t3741, t3742, t3745, t3749, t3751, t3752)
}
