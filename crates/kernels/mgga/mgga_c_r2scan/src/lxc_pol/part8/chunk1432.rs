//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1432/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1432<F: Float>(t22970: F, t25215: F, t27229: F, t27232: F, t27234: F, t27257: F, t31160: F, t31162: F, t34686: F, t34689: F, t34693: F, t34696: F, t34700: F, t34704: F, t34713: F, t8741: F) -> (F,) {
    let t34715 = -0.34930954652346593433e-1 * t34686 - 0.1047928639570397803e0 * t34689 + 0.17465477326173296717e-1 * t34693 + 0.34930954652346593433e-1 * t34696 + 0.17465477326173296717e-1 * t34700 + 0.34930954652346593433e-1 * t31160 + 0.52396431978519890152e-1 * t34704 + 0.1047928639570397803e0 * t31162 - t22970 + 0.15602799132097683414e1 * t25215 * t27257 * t8741 + t27229 - 0.66252377323950705547e1 * t27232 + 0.29451592179239371316e0 * t27234 - 0.16463622957338778996e-1 * t34713;
    (t34715,)
}
