//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1365/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1365<F: Float>(t10150: F, t1600: F, t10277: F, t378: F, t5: F, t26370: F, t159: F, t170: F, t21048: F, t21054: F, t21065: F, t21069: F, t21088: F, t21091: F, t21094: F, t26367: F, t26368: F, t28469: F, t28471: F, t32131: F) -> (F, F, F) {
    let t33395 = t1600 * t10150;
    let t33405 = t5 * t378 * t10277;
    let t33413 = 0.300153217574e-1 * t26370;
    let t33414 = t21048 - 0.16265371950452609763e-1 * t21054 - t26367 + 0.127022098e-2 * t28469 - 0.33872559466666666667e-2 * t28471 - t26368 - t21065 + t21069 - t21088 - t21091 + 0.285764e-1 * t159 * t32131 * t170 - t33413 - t21094;
    (t33395, t33405, t33414)
}
