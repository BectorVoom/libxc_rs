//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 481/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk481<F: Float>(t1001: F, t3040: F, t286: F, t1005: F, t285: F, t2867: F, t2870: F, t2872: F, t2879: F, t2882: F, t2885: F, t2891: F, t2896: F, t2901: F, t2905: F, t2913: F, t293: F, t984: F, t991: F, t996: F) -> (F, F) {
    let t3041 = t1001 * t3040;
    let t3042 = t286 * t3041;
    let t3045 = F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t2867 * t293 - t2870 / F::cast_from(54.0_f64) - t2872 * t996 / F::cast_from(54.0_f64) + t984 * t1005 / F::cast_from(18.0_f64) - t2879 + t2882 / F::cast_from(432.0_f64) - t2885 / F::cast_from(144.0_f64) + t991 * t2891 / F::cast_from(216.0_f64) - t991 * t2896 / F::cast_from(144.0_f64) - t991 * t2901 / F::cast_from(144.0_f64) + t991 * t2905 / F::cast_from(288.0_f64) + t285 * t2913 / F::cast_from(48.0_f64) - t285 * t3042 / F::cast_from(96.0_f64);
    (t3041, t3045)
}
