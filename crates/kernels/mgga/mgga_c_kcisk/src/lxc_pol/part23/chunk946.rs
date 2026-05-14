//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 946/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk946<F: Float>(t12929: F, t12931: F, t12933: F, t19100: F, t19104: F, t19116: F, t19125: F, t19129: F, t19192: F, t19199: F, t19217: F, t1175: F, t13244: F, t13247: F, t13253: F, t1355: F, t19144: F, t19165: F, t19170: F, t19173: F, t19182: F, t19185: F, t2083: F, t306: F, t3559: F, t3587: F, t3599: F, t3602: F, t5662: F, t5684: F, t5687: F) -> (F,) {
    let t19218 = 0.1651e-1 * t19116 - 0.24765e-1 * t19129 + 0.1982e-1 * t19192 - 0.36688888888888888888e-2 * t12929 + 0.13758333333333333333e-2 * t12931 + 0.9172222222222222222e-3 * t12933 - 0.18344444444444444444e-2 * t19100 - 0.55033333333333333333e-2 * t19104 + t19199 - 0.27516666666666666667e-2 * t19125 + t19217;
    let t19221 = 3.0 / 16.0 * t13244 * t19165 - t13247 * t5662 / 4.0 - t3599 * t19170 / 4.0 - t3599 * t19173 / 8.0 + t13253 * t2083 / 4.0 + t3602 * t5684 / 2.0 + t1355 * t19144 / 4.0 - t19182 * t3559 / 8.0 + t19185 * t1175 / 2.0 + t5687 * t3587 / 4.0 + t306 * t19218 / 2.0;
    (t19221,)
}
