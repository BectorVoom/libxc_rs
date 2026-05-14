//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 392/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk392<F: Float>(t1919: F, t1920: F, t2063: F, t2505: F, t673: F, t140: F, t1470: F, t1918: F, t2517: F, t2521: F, t2543: F, t479: F, t709: F, t725: F, t716: F, t736: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t2551 = t1919 * t1920 * t2063;
    let t2554 = t673 * t2505;
    let t2558 = 0.619125e-2 * t2543 * t709 + 0.9286875e-2 * t725 * t2517 - 0.619125e-2 * t725 * t2521 - t1918 - 0.26531111111111111111e-1 * t1470 * t2551 - 0.39796666666666666666e-1 * t140 * t479 * t2554;
    let t2559 = t2558 * t716;
    let t2560 = t2559 * sigma2;
    let t2561 = t2560 * t736;
    (t2551, t2554, t2558, t2559, t2560, t2561)
}
